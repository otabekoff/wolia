//! Application entry point and event loop.

use anyhow::Result;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use wolia_core::Document;
use wolia_platform::window::WindowConfig;
use wolia_render::{Quad, QuadRenderer};

use crate::workspace::Workspace;

/// UI layout constants
const TOOLBAR_HEIGHT: f32 = 48.0;
const SIDEBAR_WIDTH: f32 = 250.0;
const STATUS_BAR_HEIGHT: f32 = 24.0;
const PAPER_WIDTH: f32 = 816.0; // US Letter width in pixels at 96 DPI
const PAPER_HEIGHT: f32 = 1056.0; // US Letter height in pixels at 96 DPI
const PAPER_MARGIN: f32 = 40.0;

/// Run the Wolia Write application.
pub fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = WriteApp::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

/// The Wolia Write application.
struct WriteApp {
    /// The main window.
    window: Option<Arc<Window>>,
    /// The current workspace.
    workspace: Option<Workspace>,
    /// GPU surface for rendering.
    surface: Option<wgpu::Surface<'static>>,
    /// GPU device.
    device: Option<wgpu::Device>,
    /// GPU queue.
    queue: Option<wgpu::Queue>,
    /// Surface configuration.
    surface_config: Option<wgpu::SurfaceConfiguration>,
    /// Quad renderer for UI.
    quad_renderer: Option<QuadRenderer>,
    /// Current window size.
    window_size: (u32, u32),
}

impl WriteApp {
    fn new() -> Self {
        Self {
            window: None,
            workspace: None,
            surface: None,
            device: None,
            queue: None,
            surface_config: None,
            quad_renderer: None,
            window_size: (1400, 900),
        }
    }

    fn build_ui(&self) -> Vec<Quad> {
        let (w, h) = (self.window_size.0 as f32, self.window_size.1 as f32);
        let mut quads = Vec::new();

        // Toolbar background
        quads.push(Quad::new(
            0.0,
            0.0,
            w,
            TOOLBAR_HEIGHT,
            [0.96, 0.96, 0.96, 1.0],
        ));

        // Toolbar bottom border
        quads.push(Quad::new(
            0.0,
            TOOLBAR_HEIGHT - 1.0,
            w,
            1.0,
            [0.85, 0.85, 0.85, 1.0],
        ));

        // Toolbar buttons (File, Edit, View, Insert, Format, Tools, Help)
        let button_labels = ["File", "Edit", "View", "Insert", "Format", "Tools", "Help"];
        let mut x = 8.0;
        for _ in button_labels {
            // Button background (hover state would change this)
            quads.push(Quad::new(x, 8.0, 60.0, 32.0, [0.92, 0.92, 0.92, 1.0]));
            x += 68.0;
        }

        // Sidebar background
        quads.push(Quad::new(
            0.0,
            TOOLBAR_HEIGHT,
            SIDEBAR_WIDTH,
            h - TOOLBAR_HEIGHT - STATUS_BAR_HEIGHT,
            [0.95, 0.95, 0.95, 1.0],
        ));

        // Sidebar right border
        quads.push(Quad::new(
            SIDEBAR_WIDTH - 1.0,
            TOOLBAR_HEIGHT,
            1.0,
            h - TOOLBAR_HEIGHT - STATUS_BAR_HEIGHT,
            [0.85, 0.85, 0.85, 1.0],
        ));

        // Sidebar header
        quads.push(Quad::new(
            0.0,
            TOOLBAR_HEIGHT,
            SIDEBAR_WIDTH,
            40.0,
            [0.92, 0.92, 0.92, 1.0],
        ));

        // Document area background
        let doc_x = SIDEBAR_WIDTH;
        let doc_y = TOOLBAR_HEIGHT;
        let doc_w = w - SIDEBAR_WIDTH;
        let doc_h = h - TOOLBAR_HEIGHT - STATUS_BAR_HEIGHT;
        quads.push(Quad::new(
            doc_x,
            doc_y,
            doc_w,
            doc_h,
            [0.85, 0.85, 0.85, 1.0],
        ));

        // Paper (centered in document area)
        let paper_scale = 0.6; // Scale down for display
        let paper_w = PAPER_WIDTH * paper_scale;
        let paper_h = PAPER_HEIGHT * paper_scale;
        let paper_x = doc_x + (doc_w - paper_w) / 2.0;
        let paper_y = doc_y + PAPER_MARGIN;

        // Paper shadow
        quads.push(Quad::new(
            paper_x + 3.0,
            paper_y + 3.0,
            paper_w,
            paper_h,
            [0.0, 0.0, 0.0, 0.15],
        ));

        // Paper background
        quads.push(Quad::new(
            paper_x,
            paper_y,
            paper_w,
            paper_h,
            [1.0, 1.0, 1.0, 1.0],
        ));

        // Status bar background
        quads.push(Quad::new(
            0.0,
            h - STATUS_BAR_HEIGHT,
            w,
            STATUS_BAR_HEIGHT,
            [0.96, 0.96, 0.96, 1.0],
        ));

        // Status bar top border
        quads.push(Quad::new(
            0.0,
            h - STATUS_BAR_HEIGHT,
            w,
            1.0,
            [0.85, 0.85, 0.85, 1.0],
        ));

        quads
    }

    fn render(&mut self) {
        let Some(surface) = &self.surface else { return };
        let Some(device) = &self.device else { return };
        let Some(queue) = &self.queue else { return };
        let Some(quad_renderer) = &self.quad_renderer else {
            return;
        };

        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                if let (Some(config), Some(surface)) = (&self.surface_config, &self.surface) {
                    surface.configure(device, config);
                }
                return;
            }
            Err(e) => {
                tracing::error!("Surface error: {:?}", e);
                return;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Build and render UI
        let quads = self.build_ui();
        let (w, h) = (self.window_size.0 as f32, self.window_size.1 as f32);

        quad_renderer.render(
            &mut encoder,
            &view,
            queue,
            &quads,
            w,
            h,
            Some(wgpu::Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0,
            }),
        );

        queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}

impl ApplicationHandler for WriteApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let config = WindowConfig::new("Wolia Write").with_size(1400.0, 900.0);
            let attrs = config.to_window_attributes();

            match event_loop.create_window(attrs) {
                Ok(window) => {
                    tracing::info!("Window created");
                    let window = Arc::new(window);

                    // Initialize wgpu
                    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                        backends: wgpu::Backends::all(),
                        ..Default::default()
                    });

                    let surface = instance.create_surface(window.clone()).unwrap();

                    let adapter = pollster::block_on(instance.request_adapter(
                        &wgpu::RequestAdapterOptions {
                            power_preference: wgpu::PowerPreference::default(),
                            compatible_surface: Some(&surface),
                            force_fallback_adapter: false,
                        },
                    ))
                    .expect("Failed to find GPU adapter");

                    let (device, queue) = pollster::block_on(adapter.request_device(
                        &wgpu::DeviceDescriptor {
                            label: Some("Wolia Device"),
                            required_features: wgpu::Features::empty(),
                            required_limits: wgpu::Limits::default(),
                            memory_hints: wgpu::MemoryHints::default(),
                        },
                        None,
                    ))
                    .expect("Failed to create device");

                    let size = window.inner_size();
                    self.window_size = (size.width, size.height);

                    let surface_caps = surface.get_capabilities(&adapter);
                    let format = surface_caps.formats[0];

                    let surface_config = wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format,
                        width: size.width.max(1),
                        height: size.height.max(1),
                        present_mode: wgpu::PresentMode::AutoVsync,
                        alpha_mode: wgpu::CompositeAlphaMode::Auto,
                        view_formats: vec![],
                        desired_maximum_frame_latency: 2,
                    };
                    surface.configure(&device, &surface_config);

                    // Create quad renderer
                    let quad_renderer = QuadRenderer::new(&device, format);

                    // Create a new workspace with an empty document
                    self.workspace = Some(Workspace::new(Document::new()));
                    self.surface = Some(surface);
                    self.device = Some(device);
                    self.queue = Some(queue);
                    self.surface_config = Some(surface_config);
                    self.quad_renderer = Some(quad_renderer);
                    self.window = Some(window);
                }
                Err(e) => {
                    tracing::error!("Failed to create window: {}", e);
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Close requested, exiting");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                tracing::debug!("Window resized to {:?}", size);
                self.window_size = (size.width, size.height);
                if let (Some(surface), Some(device), Some(config)) =
                    (&self.surface, &self.device, &mut self.surface_config)
                {
                    config.width = size.width.max(1);
                    config.height = size.height.max(1);
                    surface.configure(device, config);
                }
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            WindowEvent::KeyboardInput { .. } => {
                // TODO: Handle keyboard input
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
