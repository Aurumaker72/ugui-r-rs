#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window}
};


use wgpu::{SurfaceConfiguration, TextureUsages};
use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::DeviceEvents;
use winit::keyboard::Key;
use winit::window::{WindowBuilder, WindowButtons};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,
}


impl State {
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
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,

        }
    }
}

pub async fn run() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    let windows = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(LogicalSize::new(300.0, 300.0))
        .build(&event_loop)
        .unwrap();

    let mut size = windows.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);


    //let surface = unsafe { instance.create_surface(&window) }.unwrap();

    event_loop.listen_device_events(DeviceEvents::Always);
    let state = State::new(windows).await;
    event_loop.run(move |event, elwt| {
        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::KeyboardInput {
                    event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                } => match key.as_ref() {
                    Key::Character("F" | "f") => {
                        let buttons = state.window.enabled_buttons();
                        state.window.set_enabled_buttons(buttons ^ WindowButtons::CLOSE);
                    }
                    Key::Character("G" | "g") => {
                        let buttons = state.window.enabled_buttons();
                        state.window.set_enabled_buttons(buttons ^ WindowButtons::MAXIMIZE);
                    }
                    Key::Character("H" | "h") => {
                        let buttons = state.window.enabled_buttons();
                        state.window.set_enabled_buttons(buttons ^ WindowButtons::MINIMIZE);
                    }
                    _ => (),
                },
                WindowEvent::CloseRequested if window_id == state.window.id() => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    //fill::fill_window(&window);
                }
                _ => (),
            }
        }
    })

}