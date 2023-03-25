use std::iter;

use wgpu::util::DeviceExt;
use wgpu;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use image::{RgbaImage, Rgba, DynamicImage};

pub mod vertex;
pub mod cli;
// mod texture_wire;
mod texture;
mod geometry;
mod camera;

// #[rustfmt::skip]
// pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
//     1.0, 0.0, 0.0, 0.0,
//     0.0, 1.0, 0.0, 0.0,
//     0.0, 0.0, 0.5, 0.0,
//     0.0, 0.0, 0.5, 1.0,
// );

// struct Camera {
//     eye: cgmath::Point3<f32>,
//     target: cgmath::Point3<f32>,
//     up: cgmath::Vector3<f32>,
//     aspect: f32,
//     fovy: f32,
//     znear: f32,
//     zfar: f32,
// }

// impl Camera {
//     fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
//         let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
//         let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
//         proj * view
//     }
// }

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(
        &mut self,
        camera: &camera::Camera,
        projection: &camera::Projection,
        model_view: &camera::ModelView
    ) {
        // self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()
            * model_view.calc_matrix()).into();
    }
}

// struct CameraController {
//     speed: f32,
//     is_up_pressed: bool,
//     is_down_pressed: bool,
//     is_forward_pressed: bool,
//     is_backward_pressed: bool,
//     is_left_pressed: bool,
//     is_right_pressed: bool,
// }

// impl CameraController {
//     fn new(speed: f32) -> Self {
//         Self {
//             speed,
//             is_up_pressed: false,
//             is_down_pressed: false,
//             is_forward_pressed: false,
//             is_backward_pressed: false,
//             is_left_pressed: false,
//             is_right_pressed: false,
//         }
//     }

//     fn process_events(&mut self, event: &WindowEvent) -> bool {
//         match event {
//             WindowEvent::KeyboardInput {
//                 input:
//                     KeyboardInput {
//                         state,
//                         virtual_keycode: Some(keycode),
//                         ..
//                     },
//                 ..
//             } => {
//                 let is_pressed = *state == ElementState::Pressed;
//                 match keycode {
//                     VirtualKeyCode::Space => {
//                         self.is_up_pressed = is_pressed;
//                         true
//                     }
//                     VirtualKeyCode::LShift => {
//                         self.is_down_pressed = is_pressed;
//                         true
//                     }
//                     VirtualKeyCode::W | VirtualKeyCode::Up => {
//                         self.is_forward_pressed = is_pressed;
//                         true
//                     }
//                     VirtualKeyCode::A | VirtualKeyCode::Left => {
//                         self.is_left_pressed = is_pressed;
//                         true
//                     }
//                     VirtualKeyCode::S | VirtualKeyCode::Down => {
//                         self.is_backward_pressed = is_pressed;
//                         true
//                     }
//                     VirtualKeyCode::D | VirtualKeyCode::Right => {
//                         self.is_right_pressed = is_pressed;
//                         true
//                     }
//                     _ => false,
//                 }
//             }
//             _ => false,
//         }
//     }

//     fn update_camera(&self, camera: &mut Camera) {
//         use cgmath::InnerSpace;
//         let forward = camera.target - camera.eye;
//         let forward_norm = forward.normalize();
//         let forward_mag = forward.magnitude();

//         // Prevents glitching when camera gets too close to the
//         // center of the scene.
//         if self.is_forward_pressed && forward_mag > self.speed {
//             camera.eye += forward_norm * self.speed;
//         }
//         if self.is_backward_pressed {
//             camera.eye -= forward_norm * self.speed;
//         }

//         let right = forward_norm.cross(camera.up);

//         // Redo radius calc in case the up/ down is pressed.
//         let forward = camera.target - camera.eye;
//         let forward_mag = forward.magnitude();

//         if self.is_right_pressed {
//             // Rescale the distance between the target and eye so
//             // that it doesn't change. The eye therefore still
//             // lies on the circle made by the target and eye.
//             camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
//         }
//         if self.is_left_pressed {
//             camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
//         }
//     }
// }

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    #[allow(dead_code)]
    // diffuse_texture: texture_wire::Texture,
    diffuse_bind_group: wgpu::BindGroup,
    ctab_bind_group: wgpu::BindGroup,
    depth: texture::Depth,
    // NEW!
    camera: camera::Camera,
    projection: camera::Projection,
    model_view: camera::ModelView,
    camera_controller: camera::CameraController,
    mouse_pressed: bool,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
}

impl State {
    async fn new(
        window: &Window,
        args: cli::Args
    ) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default()
            // dx12_shader_compiler: wgpu::Dx12Compiler::Fxc
        });
        let surface = (unsafe { instance.create_surface(window) })
            .unwrap_or_else(|error|{
                panic!("Error creating surface, {}", error.to_string())
            });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    // features: wgpu::Features::empty(),
                    features: wgpu::Features::POLYGON_MODE_LINE,
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None, // Trace path
            )
            .await
            .unwrap();

        // let config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: surface.get_supported_formats(&adapter)[0],
        //     width: size.width,
        //     height: size.height,
        //     present_mode: wgpu::PresentMode::Fifo,
        //     alpha_mode: wgpu::CompositeAlphaMode::Auto,
        // };

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };


        surface.configure(&device, &config);

        // Create wire texture

        let sizewire = 128;
        let sizewire1 = sizewire - 1;

        let mut wires = RgbaImage::new(sizewire, sizewire);

        // let rgba = Rgba([255, 255, 255, 255]);
        let rgba = Rgba([255, 255, 255, 255]);

        for x in 0..sizewire {
            // wires.put_pixel(x, sizewire1 - x, rgba);
            wires.put_pixel(0, x, rgba);
            wires.put_pixel(x, 0, rgba);
            wires.put_pixel(sizewire1, x, rgba);
            wires.put_pixel(x, sizewire1, rgba);
        }

        let diffuse_texture =
            texture::Texture::from_image(&device, &queue,
            &DynamicImage::ImageRgba8(wires), "Wire Frame").unwrap();

        let diffuse_bind_group_layout = diffuse_texture.bind_group_layout;
        let diffuse_bind_group = diffuse_texture.bind_group;

        // Create color table texture

        let size_ctab = 6;
        let mut ctab = RgbaImage::new(size_ctab, 1);

        ctab.put_pixel(0, 0, Rgba([255, 0, 255, 255]));
        ctab.put_pixel(1, 0, Rgba([0, 0, 255, 255]));
        ctab.put_pixel(2, 0, Rgba([0, 255, 255, 255]));
        ctab.put_pixel(3, 0, Rgba([0, 255, 0, 255]));
        ctab.put_pixel(4, 0, Rgba([255, 255, 0, 255]));
        ctab.put_pixel(5, 0, Rgba([255, 0, 0, 255]));

        let ctab_texture =
            texture::Texture::from_image(&device, &queue,
            &DynamicImage::ImageRgba8(ctab), "Color Table").unwrap();

        let ctab_bind_group_layout = ctab_texture.bind_group_layout;
        let ctab_bind_group = ctab_texture.bind_group;

        let depth = texture::Depth::create(&device, &config, "depth_texture");
    
        // let diffuse_bytes = include_bytes!("happy-tree.png");
        // let diffuse_texture =
        //     texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        // let texture_bind_group_layout =
        //     device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //         entries: &[
        //             wgpu::BindGroupLayoutEntry {
        //                 binding: 0,
        //                 visibility: wgpu::ShaderStages::FRAGMENT,
        //                 ty: wgpu::BindingType::Texture {
        //                     multisampled: false,
        //                     view_dimension: wgpu::TextureViewDimension::D2,
        //                     sample_type: wgpu::TextureSampleType::Float { filterable: true },
        //                 },
        //                 count: None,
        //             },
        //             wgpu::BindGroupLayoutEntry {
        //                 binding: 1,
        //                 visibility: wgpu::ShaderStages::FRAGMENT,
        //                 ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        //                 count: None,
        //             },
        //         ],
        //         label: Some("texture_bind_group_layout"),
        //     });

        // let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &texture_bind_group_layout,
        //     entries: &[
        //         wgpu::BindGroupEntry {
        //             binding: 0,
        //             resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
        //         },
        //         wgpu::BindGroupEntry {
        //             binding: 1,
        //             resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
        //         },
        //     ],
        //     label: Some("diffuse_bind_group"),
        // });

        // let camera = camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let model_view = camera::ModelView::new(
            cgmath::Deg(0.0), cgmath::Deg(0.0));
        let camera = camera::Camera::new(
            (0.0, 0.0, 3.0), cgmath::Deg(-90.0), cgmath::Deg(0.0));
        let projection = camera::Projection::new(
            config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
        let camera_controller = camera::CameraController::new(4.0, 0.4);

        // ...

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection, &model_view); // UPDATED!

        // let camera = Camera {
        //     eye: (0.0, 1.0, 2.0).into(),
        //     target: (0.0, 0.0, 0.0).into(),
        //     up: cgmath::Vector3::unit_y(),
        //     aspect: config.width as f32 / config.height as f32,
        //     fovy: 45.0,
        //     znear: 0.1,
        //     zfar: 100.0,
        // };
        // let camera_controller = CameraController::new(0.2);

        // let mut camera_uniform = CameraUniform::new();
        // camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &diffuse_bind_group_layout,
                    &camera_bind_group_layout,
                    &ctab_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                // entry_point: "fs_main",
                entry_point: &args.frag_entry,
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: args.front_face,
                cull_mode: args.cull_mode,
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: args.polygon_mode,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // depth_stencil: None,
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Depth::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(), // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let (vertexes, indexes) =
            args.geometry.make();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertexes),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indexes),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indexes.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            // diffuse_texture,
            diffuse_bind_group,
            ctab_bind_group,
            depth,
            camera,
            projection,
            model_view,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            mouse_pressed: false,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.projection.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth = texture::Depth::create(&self.device, &self.config, "depth_texture");
            // self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                // self.camera_controller.process_scroll(delta);
                // self.camera_controller.process_mouse(delta, delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    fn update(&mut self, dt: std::time::Duration) {
        self.camera_controller.update_model_view(&mut self.model_view, dt);
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection,
            &self.model_view);
        // println!("{:?}", self.camera_uniform);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                // depth_stencil_attachment: None,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(2, &self.ctab_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run(args: cli::Args) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(&window, args).await;
    let mut last_render_time = instant::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                state.update(dt);
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
