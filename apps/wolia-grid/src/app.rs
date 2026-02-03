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
const FORMULA_BAR_HEIGHT: f32 = 32.0;
const ROW_HEADER_WIDTH: f32 = 50.0;
const COLUMN_HEADER_HEIGHT: f32 = 24.0;
const STATUS_BAR_HEIGHT: f32 = 24.0;
const CELL_WIDTH: f32 = 100.0;
const CELL_HEIGHT: f32 = 24.0;

/// Run the Wolia Grid application.
pub fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = GridApp::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

/// The Wolia Grid application.
struct GridApp {
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

impl GridApp {
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

        // Toolbar buttons
        let mut x = 8.0;
        for _ in 0..7 {
            quads.push(Quad::new(x, 8.0, 60.0, 32.0, [0.92, 0.92, 0.92, 1.0]));
            x += 68.0;
        }

        // Formula bar background
        let formula_y = TOOLBAR_HEIGHT;
        quads.push(Quad::new(
            0.0,
            formula_y,
            w,
            FORMULA_BAR_HEIGHT,
            [0.98, 0.98, 0.98, 1.0],
        ));

        // Formula bar bottom border
        quads.push(Quad::new(
            0.0,
            formula_y + FORMULA_BAR_HEIGHT - 1.0,
            w,
            1.0,
            [0.85, 0.85, 0.85, 1.0],
        ));

        // Name box (cell reference display)
        quads.push(Quad::new(
            4.0,
            formula_y + 4.0,
            ROW_HEADER_WIDTH + 40.0,
            FORMULA_BAR_HEIGHT - 8.0,
            [1.0, 1.0, 1.0, 1.0],
        ));

        // Formula input area
        quads.push(Quad::new(
            ROW_HEADER_WIDTH + 50.0,
            formula_y + 4.0,
            w - ROW_HEADER_WIDTH - 60.0,
            FORMULA_BAR_HEIGHT - 8.0,
            [1.0, 1.0, 1.0, 1.0],
        ));

        // Column headers background
        let header_y = formula_y + FORMULA_BAR_HEIGHT;
        quads.push(Quad::new(
            ROW_HEADER_WIDTH,
            header_y,
            w - ROW_HEADER_WIDTH,
            COLUMN_HEADER_HEIGHT,
            [0.95, 0.95, 0.95, 1.0],
        ));

        // Row headers background
        let grid_y = header_y + COLUMN_HEADER_HEIGHT;
        let grid_height = h - grid_y - STATUS_BAR_HEIGHT;
        quads.push(Quad::new(
            0.0,
            grid_y,
            ROW_HEADER_WIDTH,
            grid_height,
            [0.95, 0.95, 0.95, 1.0],
        ));

        // Corner cell (top-left)
        quads.push(Quad::new(
            0.0,
            header_y,
            ROW_HEADER_WIDTH,
            COLUMN_HEADER_HEIGHT,
            [0.90, 0.90, 0.90, 1.0],
        ));

        // Column header cells
        let num_cols = ((w - ROW_HEADER_WIDTH) / CELL_WIDTH) as usize + 1;
        for i in 0..num_cols {
            let col_x = ROW_HEADER_WIDTH + (i as f32 * CELL_WIDTH);
            if col_x < w {
                // Column header cell
                quads.push(Quad::new(
                    col_x,
                    header_y,
                    CELL_WIDTH,
                    COLUMN_HEADER_HEIGHT,
                    [0.95, 0.95, 0.95, 1.0],
                ));
                // Right border
                quads.push(Quad::new(
                    col_x + CELL_WIDTH - 1.0,
                    header_y,
                    1.0,
                    COLUMN_HEADER_HEIGHT,
                    [0.82, 0.82, 0.82, 1.0],
                ));
            }
        }

        // Grid cells background (white)
        quads.push(Quad::new(
            ROW_HEADER_WIDTH,
            grid_y,
            w - ROW_HEADER_WIDTH,
            grid_height,
            [1.0, 1.0, 1.0, 1.0],
        ));

        // Draw cell grid lines (vertical)
        for i in 0..=num_cols {
            let col_x = ROW_HEADER_WIDTH + (i as f32 * CELL_WIDTH);
            if col_x <= w {
                quads.push(Quad::new(
                    col_x,
                    grid_y,
                    1.0,
                    grid_height,
                    [0.85, 0.85, 0.85, 1.0],
                ));
            }
        }

        // Draw cell grid lines (horizontal) and row headers
        let num_rows = (grid_height / CELL_HEIGHT) as usize + 1;
        for i in 0..=num_rows {
            let row_y = grid_y + (i as f32 * CELL_HEIGHT);
            if row_y <= h - STATUS_BAR_HEIGHT {
                // Horizontal line
                quads.push(Quad::new(
                    ROW_HEADER_WIDTH,
                    row_y,
                    w - ROW_HEADER_WIDTH,
                    1.0,
                    [0.85, 0.85, 0.85, 1.0],
                ));

                // Row header cell
                if i < num_rows {
                    quads.push(Quad::new(
                        0.0,
                        row_y,
                        ROW_HEADER_WIDTH - 1.0,
                        CELL_HEIGHT,
                        [0.95, 0.95, 0.95, 1.0],
                    ));
                    // Row header right border
                    quads.push(Quad::new(
                        ROW_HEADER_WIDTH - 1.0,
                        row_y,
                        1.0,
                        CELL_HEIGHT,
                        [0.82, 0.82, 0.82, 1.0],
                    ));
                }
            }
        }

        // Selected cell highlight (A1)
        quads.push(Quad::new(
            ROW_HEADER_WIDTH + 1.0,
            grid_y + 1.0,
            CELL_WIDTH - 2.0,
            CELL_HEIGHT - 1.0,
            [0.26, 0.52, 0.96, 0.15],
        ));
        // Selected cell border
        quads.push(Quad::new(
            ROW_HEADER_WIDTH,
            grid_y,
            CELL_WIDTH,
            2.0,
            [0.26, 0.52, 0.96, 1.0],
        )); // top
        quads.push(Quad::new(
            ROW_HEADER_WIDTH,
            grid_y + CELL_HEIGHT - 2.0,
            CELL_WIDTH,
            2.0,
            [0.26, 0.52, 0.96, 1.0],
        )); // bottom
        quads.push(Quad::new(
            ROW_HEADER_WIDTH,
            grid_y,
            2.0,
            CELL_HEIGHT,
            [0.26, 0.52, 0.96, 1.0],
        )); // left
        quads.push(Quad::new(
            ROW_HEADER_WIDTH + CELL_WIDTH - 2.0,
            grid_y,
            2.0,
            CELL_HEIGHT,
            [0.26, 0.52, 0.96, 1.0],
        )); // right

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
                r: 0.94,
                g: 0.94,
                b: 0.94,
                a: 1.0,
            }),
        );

        queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}

impl ApplicationHandler for GridApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let config = WindowConfig::new("Wolia Grid")
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
