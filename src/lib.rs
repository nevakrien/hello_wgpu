use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::window::Window;

#[derive(Clone, Copy,Debug)]
enum Mode {
    Mode1,
    Mode2,
}

fn switch_mode(current_mode: Mode) -> Mode {
    //println!("Current mode: {:?}", current_mode);
    match current_mode {
        Mode::Mode1 => Mode::Mode2,
        Mode::Mode2 => Mode::Mode1,
    }
}

//use std::path::Path;
//use std::fs;

macro_rules! load_pipline {
    ($shader_file:expr, $device:expr, $config:expr) => {{
        // Include the shader code at compile time
        let shader_code = include_str!($shader_file);
        // Call the modified function with the included shader code
        load_pipeline_from_shader(shader_code, $device, $config)
    }};
}

fn load_pipeline_from_shader(shader_code: &str,device: &wgpu::Device,config: &wgpu::SurfaceConfiguration,) ->wgpu::RenderPipeline{
        //shader stuff
        // Read the shader file at runtime
        // let shader_path = std::path::Path::new(shader_file);
        // let shader_code = std::fs::read_to_string(shader_path).expect("Failed to read shader file");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_code.into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,//Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        return render_pipeline;
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,

    color: wgpu::Color,

    render_pipeline: wgpu::RenderPipeline,
    render_pipeline2:wgpu::RenderPipeline,
    mode: Mode


}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window, so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        
        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| {
                // Check if this adapter supports our surface
                adapter.is_surface_supported(&surface)
            })
            .next()
            .unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        
        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
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
        //let modes = &surface_caps.present_modes; //optional

        let color=wgpu::Color {r: 0.1,g: 0.2,b: 0.3,a: 1.0,};

        let render_pipeline=load_pipline!("shader.wgsl",&device,&config);
        let render_pipeline2=load_pipline!("my_shader.wgsl",&device,&config);
        // //shader stuff
        // let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        // let render_pipeline_layout =
        //     device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //         label: Some("Render Pipeline Layout"),
        //         bind_group_layouts: &[],
        //         push_constant_ranges: &[],
        //     });


        // let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //     label: Some("Render Pipeline"),
        //     layout: Some(&render_pipeline_layout),
        //     vertex: wgpu::VertexState {
        //         module: &shader,
        //         entry_point: "vs_main", // 1.
        //         buffers: &[], // 2.
        //     },
        //     fragment: Some(wgpu::FragmentState { // 3.
        //         module: &shader,
        //         entry_point: "fs_main",
        //         targets: &[Some(wgpu::ColorTargetState { // 4.
        //             format: config.format,
        //             blend: Some(wgpu::BlendState::REPLACE),
        //             write_mask: wgpu::ColorWrites::ALL,
        //         })],
        //     }),

        //     primitive: wgpu::PrimitiveState {
        //         topology: wgpu::PrimitiveTopology::TriangleList, // 1.
        //         strip_index_format: None,
        //         front_face: wgpu::FrontFace::Ccw, // 2.
        //         cull_mode: Some(wgpu::Face::Back),
        //         // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
        //         polygon_mode: wgpu::PolygonMode::Fill,
        //         // Requires Features::DEPTH_CLIP_CONTROL
        //         unclipped_depth: false,
        //         // Requires Features::CONSERVATIVE_RASTERIZATION
        //         conservative: false,
        //     },
        //     depth_stencil: None, // 1.
        //     multisample: wgpu::MultisampleState {
        //         count: 1, // 2.
        //         mask: !0, // 3.
        //         alpha_to_coverage_enabled: false, // 4.
        //     },
        //     multiview: None, // 5.
        // });

        let mode=Mode::Mode1;

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            color,
            render_pipeline,
            render_pipeline2,
            mode,
        }

    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }


    fn input(&mut self, event: &WindowEvent) -> bool {
        match event{
            WindowEvent::CursorMoved{ position,.. } => {
                // let normalized_x = position.x as f64 / self.size.width as f64;
                // let normalized_y = position.y as f64 / self.size.height as f64;
                    
                // self.color = wgpu::Color {
                //     r: normalized_x,
                //     g: normalized_y,
                //     b: 0.3, // Fixed blue value for demonstration
                //     a: 1.0,
                // };

                //println!("Updated color: r={}, g={}, b={}", self.color.r, self.color.g, self.color.b);

                true
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
            ..} => {
                
                self.mode=switch_mode(self.mode);
                true
            },

            _ => false  
        }
        //false
        //todo!()
    }

    fn update(&mut self) {
        //todo!()
    }
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Acquire the current texture from the swap chain (surface)
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder for recording rendering commands
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Begin the render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, // Use the view of the current texture
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let render_pipeline = match self.mode{
                Mode::Mode1=>&self.render_pipeline,
                Mode::Mode2=>&self.render_pipeline2,
            };

            render_pass.set_pipeline(render_pipeline); // 2.
            render_pass.draw(0..3, 0..1); // 3.

        } // The render pass is dropped here, ending it

        // Submit the command buffer to the queue
        self.queue.submit(std::iter::once(encoder.finish()));
        // Present the current texture to the screen
        output.present();

        Ok(())
    }

}

 

 

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            //console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
            //log::info!("Setup of logging ready");

            //use web_sys::console;
            //console::log_1(&JsValue::from_str("useless from web_sys"));
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


    log::info!("window made");
    let mut state = State::new(window).await;


    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => match event {
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
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &&mut so we have to dereference it twice
                    state.resize(**new_inner_size);
                },
                _ => {state.input(event);}
            },

            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once unless we manually
                // request it.
                state.window().request_redraw();
            }

            _ => {}
        }
    });
}

