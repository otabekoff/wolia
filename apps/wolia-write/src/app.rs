//! Application entry point and event loop.

use anyhow::Result;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use wolia_assets::icons::IconManager;
use wolia_core::Document;
use wolia_platform::window::WindowConfig;
use wolia_render::{IconRenderer, Quad, QuadRenderer};

use crate::automation::AutomationDriver;
use crate::workspace::Workspace;

/// UI layout constants
const TOOLBAR_HEIGHT: f32 = 48.0;
const SIDEBAR_WIDTH: f32 = 250.0;
const STATUS_BAR_HEIGHT: f32 = 24.0;
const PAPER_WIDTH: f32 = 816.0; // US Letter width in pixels at 96 DPI
const PAPER_HEIGHT: f32 = 1056.0; // US Letter height in pixels at 96 DPI
const PAPER_MARGIN: f32 = 40.0;

/// Run the Wolia Write application.
pub fn run(enable_automation: bool) -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = WriteApp::new(enable_automation);
    event_loop.run_app(&mut app)?;

    Ok(())
}

/// The Wolia Write application.
struct WriteApp {
    /// The main window - MUST be dropped LAST (after surface).
    window: Option<Arc<Window>>,
    /// The current workspace.
    workspace: Option<Workspace>,
    /// GPU surface for rendering - MUST be dropped BEFORE window.
    surface: Option<wgpu::Surface<'static>>,
    /// GPU device.
    device: Option<wgpu::Device>,
    /// GPU queue.
    queue: Option<wgpu::Queue>,
    /// Surface configuration.
    surface_config: Option<wgpu::SurfaceConfiguration>,
    /// Quad renderer for UI.
    quad_renderer: Option<QuadRenderer>,
    /// Icon renderer for SVG icons.
    icon_renderer: Option<IconRenderer>,
    /// Current window size.
    window_size: (u32, u32),
    /// Current mouse position.
    mouse_position: (f32, f32),
    /// Whether mouse button is pressed.
    mouse_pressed: bool,
    /// Automation driver for testing.
    automation: AutomationDriver,
}

impl WriteApp {
    fn new(enable_automation: bool) -> Self {
        Self {
            window: None,
            workspace: None,
            surface: None,
            device: None,
            queue: None,
            surface_config: None,
            quad_renderer: None,
            icon_renderer: None,
            window_size: (1400, 900),
            mouse_position: (0.0, 0.0),
            mouse_pressed: false,
            automation: AutomationDriver::new(enable_automation),
        }
    }

    /// Handle mouse movement - update button hover states.
    fn handle_mouse_move(&mut self) {
        let (mx, my) = self.mouse_position;
        if let Some(workspace) = &mut self.workspace {
            // Reset all buttons to Normal (unless they're Active)
            for category in workspace.toolbar.buttons.values_mut() {
                for button in category.iter_mut() {
                    if button.state != crate::toolbar::ButtonState::Active {
                        if button.contains_point(mx, my) {
                            if button.state != crate::toolbar::ButtonState::Hovered {
                                button.state = crate::toolbar::ButtonState::Hovered;
                            }
                        } else if button.state == crate::toolbar::ButtonState::Hovered {
                            button.state = crate::toolbar::ButtonState::Normal;
                        }
                    }
                }
            }
        }
    }

    /// Handle mouse press - activate buttons.
    fn handle_mouse_press(&mut self) {
        let (mx, my) = self.mouse_position;
        if let Some(workspace) = &mut self.workspace {
            for category in workspace.toolbar.buttons.values_mut() {
                for button in category.iter_mut() {
                    if button.contains_point(mx, my) {
                        // Toggle Active state for formatting buttons
                        if button.state == crate::toolbar::ButtonState::Active {
                            button.state = crate::toolbar::ButtonState::Normal;
                        } else {
                            button.state = crate::toolbar::ButtonState::Active;
                        }
                        tracing::info!("Button clicked: {} ({})", button.id, button.tooltip);
                    }
                }
            }
        }
    }

    /// Handle mouse release.
    fn handle_mouse_release(&mut self) {
        // For now, keep active state until clicked again
        // This is toggle behavior for formatting buttons like Bold, Italic
    }

    /// Clean up GPU resources in the correct order to prevent segfaults.
    fn cleanup(&mut self) {
        tracing::info!("Cleaning up GPU resources...");
        // Drop in correct order: renderers -> surface -> device -> window
        self.icon_renderer = None;
        self.quad_renderer = None;
        self.surface_config = None;
        self.surface = None;
        self.queue = None;
        self.device = None;
        self.workspace = None;
        self.window = None;
        tracing::info!("Cleanup complete");
    }

    fn build_ui(&self) -> Vec<Quad> {
        let (w, h) = (self.window_size.0 as f32, self.window_size.1 as f32);
        let mut quads = Vec::new();

        // 1. Toolbar Background
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

        // 2. Toolbar Buttons
        if let Some(workspace) = &self.workspace {
            use crate::toolbar::ButtonState;
            for button in workspace.toolbar.all_buttons() {
                // Determine color based on state
                let color = match button.state {
                    ButtonState::Normal => [0.92, 0.92, 0.92, 1.0], // Default grey
                    ButtonState::Hovered => [0.88, 0.88, 0.95, 1.0], // Light blue tint
                    ButtonState::Active => [0.80, 0.80, 0.90, 1.0], // Darker blue
                    ButtonState::Disabled => [0.96, 0.96, 0.96, 0.5], // Faded
                };

                quads.push(Quad::new(
                    button.x,
                    button.y,
                    button.width,
                    button.height,
                    color,
                ));
            }
        }

        // 3. Sidebar
        let mut sidebar_width = 0.0;
        if let Some(workspace) = &self.workspace {
            if workspace.sidebar.visible {
                sidebar_width = workspace.sidebar.width;
                let sidebar_height = h - TOOLBAR_HEIGHT - STATUS_BAR_HEIGHT;

                // Background
                quads.push(Quad::new(
                    0.0,
                    TOOLBAR_HEIGHT,
                    sidebar_width,
                    sidebar_height,
                    [0.95, 0.95, 0.95, 1.0],
                ));

                // Right border
                quads.push(Quad::new(
                    sidebar_width - 1.0,
                    TOOLBAR_HEIGHT,
                    1.0,
                    sidebar_height,
                    [0.85, 0.85, 0.85, 1.0],
                ));

                // Header background
                quads.push(Quad::new(
                    0.0,
                    TOOLBAR_HEIGHT,
                    sidebar_width,
                    40.0,
                    [0.92, 0.92, 0.92, 1.0],
                ));

                // Render outline items as placeholders (since we can't render text yet)
                let items = workspace.sidebar.outline.flatten();
                for (item, item_y) in items {
                    // Start below header (40px)
                    let y_pos = TOOLBAR_HEIGHT + 40.0 + item_y;
                    if y_pos > h - STATUS_BAR_HEIGHT {
                        break; // Clip to bottom
                    }

                    // Highlight selected item (if we knew which one was selected, but flatten returns item ref)
                    // For now just render simple line markers
                    let indent = 16.0 + (item.level as f32) * 16.0;

                    quads.push(Quad::new(
                        indent,
                        y_pos + 4.0,
                        120.0, // Placeholder text width
                        16.0,
                        [0.8, 0.8, 0.8, 1.0],
                    ));
                }
            }
        }

        // 4. Status Bar
        if let Some(workspace) = &self.workspace {
            if workspace.statusbar.visible {
                let sb_y = h - STATUS_BAR_HEIGHT;

                // Background
                quads.push(Quad::new(
                    0.0,
                    sb_y,
                    w,
                    STATUS_BAR_HEIGHT,
                    [0.96, 0.96, 0.96, 1.0],
                ));

                // Top border
                quads.push(Quad::new(0.0, sb_y, w, 1.0, [0.85, 0.85, 0.85, 1.0]));

                // Status indicator dot
                let (r, g, b) = workspace.statusbar.status.color_rgb();
                quads.push(Quad::new(12.0, sb_y + 8.0, 8.0, 8.0, [r, g, b, 1.0]));
            }
        }

        // 5. Document Area
        let doc_x = sidebar_width;
        let doc_y = TOOLBAR_HEIGHT;
        let doc_w = w - sidebar_width;
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

        // Build and render UI quads
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

        // Render icons on toolbar buttons
        if let (Some(icon_renderer), Some(workspace)) = (&self.icon_renderer, &self.workspace) {
            use crate::toolbar::ButtonState;

            for button in workspace.toolbar.all_buttons() {
                if icon_renderer.has_icon(&button.icon) {
                    // Choose tint based on button state
                    let tint = match button.state {
                        ButtonState::Normal => [0.3, 0.3, 0.3, 1.0],
                        ButtonState::Hovered => [0.1, 0.1, 0.4, 1.0],
                        ButtonState::Active => [0.0, 0.0, 0.6, 1.0],
                        ButtonState::Disabled => [0.6, 0.6, 0.6, 0.5],
                    };

                    // Center icon in button (icon size = 20, button size = 32)
                    let icon_size = 20.0;
                    let icon_x = button.x + (button.width - icon_size) / 2.0;
                    let icon_y = button.y + (button.height - icon_size) / 2.0;

                    icon_renderer.render_icon(
                        &mut encoder,
                        &view,
                        queue,
                        &button.icon,
                        icon_x,
                        icon_y,
                        icon_size,
                        w,
                        h,
                        tint,
                    );
                }
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}

impl ApplicationHandler for WriteApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let config = WindowConfig::new("Wolia Write")
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

                    // Try to get adapter, retry with fallback if needed
                    let mut adapter_opt = pollster::block_on(instance.request_adapter(
                        &wgpu::RequestAdapterOptions {
                            power_preference: wgpu::PowerPreference::HighPerformance,
                            compatible_surface: Some(&surface),
                            force_fallback_adapter: false,
                        },
                    ));

                    if adapter_opt.is_none() {
                        tracing::warn!(
                            "Failed to find HighPerformance adapter, trying LowPower..."
                        );
                        adapter_opt = pollster::block_on(instance.request_adapter(
                            &wgpu::RequestAdapterOptions {
                                power_preference: wgpu::PowerPreference::LowPower,
                                compatible_surface: Some(&surface),
                                force_fallback_adapter: false,
                            },
                        ));
                    }

                    if adapter_opt.is_none() {
                        tracing::warn!(
                            "Failed to find any hardware adapter, trying fallback software adapter..."
                        );
                        adapter_opt = pollster::block_on(instance.request_adapter(
                            &wgpu::RequestAdapterOptions {
                                power_preference: wgpu::PowerPreference::default(),
                                compatible_surface: Some(&surface),
                                force_fallback_adapter: true,
                            },
                        ));
                    }

                    let adapter = adapter_opt.expect("Failed to find any GPU adapter");

                    tracing::info!("Using GPU Adapter: {:?}", adapter.get_info());

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

                    // Create icon renderer and load toolbar icons
                    let mut icon_renderer = IconRenderer::new(&device, format);
                    let icon_manager = IconManager::new();

                    // Load icons for all toolbar buttons
                    let toolbar_icons = [
                        "file-plus",
                        "folder-open",
                        "save",
                        "undo",
                        "redo",
                        "scissors",
                        "copy",
                        "clipboard-paste",
                        "bold",
                        "italic",
                        "underline",
                        "strikethrough",
                        "text-align-start",
                        "text-align-center",
                        "text-align-end",
                        "text-align-justify",
                        "list",
                        "list-ordered",
                        "image",
                        "table",
                        "link",
                    ];

                    let mut loaded_count = 0;
                    for icon_name in &toolbar_icons {
                        if let Some(svg_data) = icon_manager.get(icon_name) {
                            if icon_renderer.load_icon(&device, &queue, icon_name, &svg_data, 24) {
                                loaded_count += 1;
                            }
                        }
                    }
                    tracing::info!(
                        "Loaded {}/{} toolbar icons",
                        loaded_count,
                        toolbar_icons.len()
                    );

                    // Create a new workspace with an empty document
                    let workspace = Workspace::new(Document::new());
                    tracing::info!("Workspace initialized");
                    tracing::info!(
                        "UI: Toolbar mounted ({} buttons)",
                        workspace.toolbar.all_buttons().len()
                    );
                    tracing::info!("UI: Sidebar mounted (Width: {})", workspace.sidebar.width);
                    tracing::info!("UI: StatusBar mounted");

                    self.workspace = Some(workspace);
                    self.surface = Some(surface);
                    self.device = Some(device);
                    self.queue = Some(queue);
                    self.surface_config = Some(surface_config);
                    self.quad_renderer = Some(quad_renderer);
                    self.icon_renderer = Some(icon_renderer);
                    self.window = Some(window);

                    if self.automation.enabled {
                        self.automation.load_scenario("smoke_test");
                    }
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
                self.cleanup();
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
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x as f32, position.y as f32);
                self.handle_mouse_move();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                use winit::event::{ElementState, MouseButton};
                if button == MouseButton::Left {
                    match state {
                        ElementState::Pressed => {
                            self.mouse_pressed = true;
                            self.handle_mouse_press();
                        }
                        ElementState::Released => {
                            self.mouse_pressed = false;
                            self.handle_mouse_release();
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput { .. } => {
                // TODO: Handle keyboard input
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.automation.tick() {
            tracing::info!("Automation sequence completed. Exiting.");
            self.cleanup();
            event_loop.exit();
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
