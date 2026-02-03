//! Simple 2D quad renderer for UI elements.

use wgpu::util::DeviceExt;

/// Vertex for 2D quads.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x4,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// A simple quad (rectangle) to render.
#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
}

impl Quad {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    /// Convert to vertices (2 triangles = 6 vertices).
    pub fn to_vertices(&self, screen_width: f32, screen_height: f32) -> [Vertex; 6] {
        // Convert pixel coordinates to NDC (-1 to 1)
        let x1 = (self.x / screen_width) * 2.0 - 1.0;
        let y1 = 1.0 - (self.y / screen_height) * 2.0;
        let x2 = ((self.x + self.width) / screen_width) * 2.0 - 1.0;
        let y2 = 1.0 - ((self.y + self.height) / screen_height) * 2.0;

        [
            Vertex {
                position: [x1, y1],
                color: self.color,
            },
            Vertex {
                position: [x2, y1],
                color: self.color,
            },
            Vertex {
                position: [x1, y2],
                color: self.color,
            },
            Vertex {
                position: [x1, y2],
                color: self.color,
            },
            Vertex {
                position: [x2, y1],
                color: self.color,
            },
            Vertex {
                position: [x2, y2],
                color: self.color,
            },
        ]
    }
}

/// Simple 2D quad renderer.
pub struct QuadRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    max_quads: usize,
}

impl QuadRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Quad Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("quad.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Quad Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Quad Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let max_quads = 1000;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Quad Vertex Buffer"),
            size: (max_quads * 6 * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            max_quads,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        queue: &wgpu::Queue,
        quads: &[Quad],
        screen_width: f32,
        screen_height: f32,
        clear_color: Option<wgpu::Color>,
    ) {
        if quads.is_empty() && clear_color.is_none() {
            return;
        }

        // Convert quads to vertices
        let mut vertices = Vec::with_capacity(quads.len() * 6);
        for quad in quads.iter().take(self.max_quads) {
            vertices.extend_from_slice(&quad.to_vertices(screen_width, screen_height));
        }

        if !vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        }

        let load_op = match clear_color {
            Some(color) => wgpu::LoadOp::Clear(color),
            None => wgpu::LoadOp::Load,
        };

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Quad Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: load_op,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if !vertices.is_empty() {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..vertices.len() as u32, 0..1);
        }
    }
}
