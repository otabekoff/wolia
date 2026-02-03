//! Application entry point and event loop.

use anyhow::Result;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use wolia_platform::window::WindowConfig;
use wolia_render::{Quad, QuadRenderer};

/// UI layout constants
const TOOLBAR_HEIGHT: f32 = 48.0;
const SLIDE_PANEL_WIDTH: f32 = 200.0;
const STATUS_BAR_HEIGHT: f32 = 24.0;
const SLIDE_ASPECT_RATIO: f32 = 16.0 / 9.0;
const SLIDE_THUMBNAIL_MARGIN: f32 = 12.0;

/// Run the Wolia Deck application.
pub fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = DeckApp::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

/// The Wolia Deck application.
struct DeckApp {
    /// The main window.
    window: Option<Arc<Window>>,
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

impl DeckApp {
    fn new() -> Self {
        Self {
            window: None,
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

        // Toolbar background (dark theme)
        quads.push(Quad::new(
            0.0,
            0.0,
            w,
            TOOLBAR_HEIGHT,
            [0.18, 0.18, 0.20, 1.0],
        ));

        // Toolbar bottom border
        quads.push(Quad::new(
            0.0,
            TOOLBAR_HEIGHT - 1.0,
            w,
            1.0,
            [0.25, 0.25, 0.28, 1.0],
        ));

        // Toolbar buttons
        let mut x = 8.0;
        for _ in 0..7 {
            quads.push(Quad::new(x, 10.0, 50.0, 28.0, [0.25, 0.25, 0.28, 1.0]));
            x += 58.0;
        }

        // Slide panel background (left sidebar)
        let panel_y = TOOLBAR_HEIGHT;
        let panel_height = h - TOOLBAR_HEIGHT - STATUS_BAR_HEIGHT;
        quads.push(Quad::new(
            0.0,
            panel_y,
            SLIDE_PANEL_WIDTH,
            panel_height,
            [0.12, 0.12, 0.14, 1.0],
        ));

        // Slide panel right border
        quads.push(Quad::new(
            SLIDE_PANEL_WIDTH - 1.0,
            panel_y,
            1.0,
            panel_height,
            [0.25, 0.25, 0.28, 1.0],
        ));

        // Slide thumbnails
        let thumb_width = SLIDE_PANEL_WIDTH - SLIDE_THUMBNAIL_MARGIN * 2.0;
        let thumb_height = thumb_width / SLIDE_ASPECT_RATIO;
        let mut thumb_y = panel_y + SLIDE_THUMBNAIL_MARGIN;

        for i in 0..5 {
            // Thumbnail background
            quads.push(Quad::new(
                SLIDE_THUMBNAIL_MARGIN,
                thumb_y,
                thumb_width,
                thumb_height,
                [0.2, 0.2, 0.22, 1.0],
            ));

            // Slide content (white slide preview)
            quads.push(Quad::new(
                SLIDE_THUMBNAIL_MARGIN + 4.0,
                thumb_y + 4.0,
                thumb_width - 8.0,
                thumb_height - 8.0,
                [0.95, 0.95, 0.95, 1.0],
            ));

            // First slide selected indicator
            if i == 0 {
                quads.push(Quad::new(
                    SLIDE_THUMBNAIL_MARGIN - 2.0,
                    thumb_y - 2.0,
                    thumb_width + 4.0,
                    2.0,
                    [0.26, 0.52, 0.96, 1.0],
                )); // top
                quads.push(Quad::new(
                    SLIDE_THUMBNAIL_MARGIN - 2.0,
                    thumb_y + thumb_height,
                    thumb_width + 4.0,
                    2.0,
                    [0.26, 0.52, 0.96, 1.0],
                )); // bottom
                quads.push(Quad::new(
                    SLIDE_THUMBNAIL_MARGIN - 2.0,
                    thumb_y - 2.0,
                    2.0,
                    thumb_height + 4.0,
                    [0.26, 0.52, 0.96, 1.0],
                )); // left
                quads.push(Quad::new(
                    SLIDE_THUMBNAIL_MARGIN + thumb_width,
                    thumb_y - 2.0,
                    2.0,
                    thumb_height + 4.0,
                    [0.26, 0.52, 0.96, 1.0],
                )); // right
            }

            thumb_y += thumb_height + SLIDE_THUMBNAIL_MARGIN;
        }

        // Canvas area background (dark)
        let canvas_x = SLIDE_PANEL_WIDTH;
        let canvas_y = TOOLBAR_HEIGHT;
        let canvas_w = w - SLIDE_PANEL_WIDTH;
        let canvas_h = h - TOOLBAR_HEIGHT - STATUS_BAR_HEIGHT;
        quads.push(Quad::new(
            canvas_x,
            canvas_y,
            canvas_w,
            canvas_h,
            [0.15, 0.15, 0.18, 1.0],
        ));

        // Main slide (centered, 16:9 aspect ratio)
        let slide_margin = 40.0;
        let available_w = canvas_w - slide_margin * 2.0;
        let available_h = canvas_h - slide_margin * 2.0;

        // Calculate slide size maintaining aspect ratio
        let slide_w;
        let slide_h;
        if available_w / SLIDE_ASPECT_RATIO <= available_h {
            slide_w = available_w;
            slide_h = available_w / SLIDE_ASPECT_RATIO;
        } else {
            slide_h = available_h;
            slide_w = available_h * SLIDE_ASPECT_RATIO;
        }

        let slide_x = canvas_x + (canvas_w - slide_w) / 2.0;
        let slide_y = canvas_y + (canvas_h - slide_h) / 2.0;

        // Slide shadow
        quads.push(Quad::new(
            slide_x + 4.0,
            slide_y + 4.0,
            slide_w,
            slide_h,
            [0.0, 0.0, 0.0, 0.3],
        ));

        // Main slide background
        quads.push(Quad::new(
            slide_x,
            slide_y,
            slide_w,
            slide_h,
            [1.0, 1.0, 1.0, 1.0],
        ));

        // Placeholder content on slide (title area)
        quads.push(Quad::new(
            slide_x + slide_w * 0.1,
            slide_y + slide_h * 0.15,
            slide_w * 0.8,
            slide_h * 0.08,
            [0.9, 0.9, 0.9, 1.0],
        ));

        // Placeholder content on slide (subtitle area)
        quads.push(Quad::new(
            slide_x + slide_w * 0.15,
            slide_y + slide_h * 0.28,
            slide_w * 0.7,
            slide_h * 0.04,
            [0.92, 0.92, 0.92, 1.0],
        ));

        // Placeholder content on slide (body area)
        quads.push(Quad::new(
            slide_x + slide_w * 0.1,
            slide_y + slide_h * 0.4,
            slide_w * 0.8,
            slide_h * 0.45,
            [0.96, 0.96, 0.96, 1.0],
        ));

        // Status bar background
        quads.push(Quad::new(
            0.0,
            h - STATUS_BAR_HEIGHT,
            w,
            STATUS_BAR_HEIGHT,
            [0.18, 0.18, 0.20, 1.0],
        ));

        // Status bar top border
        quads.push(Quad::new(
            0.0,
            h - STATUS_BAR_HEIGHT,
            w,
            1.0,
            [0.25, 0.25, 0.28, 1.0],
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
                r: 0.15,
                g: 0.15,
                b: 0.18,
                a: 1.0,
            }),
        );

        queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}

impl ApplicationHandler for DeckApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let config = WindowConfig::new("Wolia Deck")
                .with_size(1400.0, 900.0)
                .with_icon("wolia.png");
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
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
