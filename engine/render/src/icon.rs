//! Icon rendering with SVG rasterization.

use std::collections::HashMap;

/// A rasterized icon ready for GPU rendering.
pub struct RasterizedIcon {
    /// RGBA pixel data.
    pub pixels: Vec<u8>,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

impl RasterizedIcon {
    /// Rasterize an SVG string to RGBA pixels.
    pub fn from_svg(svg_data: &str, target_size: u32) -> Option<Self> {
        let options = resvg::usvg::Options::default();
        let tree = resvg::usvg::Tree::from_str(svg_data, &options).ok()?;

        let tree_size = tree.size();
        let scale = target_size as f32 / tree_size.width().max(tree_size.height());

        let width = (tree_size.width() * scale).ceil() as u32;
        let height = (tree_size.height() * scale).ceil() as u32;

        let mut pixmap = tiny_skia::Pixmap::new(width, height)?;

        let transform = tiny_skia::Transform::from_scale(scale, scale);
        resvg::render(&tree, transform, &mut pixmap.as_mut());

        Some(Self {
            pixels: pixmap.take(),
            width,
            height,
        })
    }
}

/// Vertex for textured quads.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexturedVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl TexturedVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
        2 => Float32x4,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TexturedVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// A cached icon texture on the GPU.
pub struct IconTexture {
    /// The GPU texture.
    pub texture: wgpu::Texture,
    /// Texture view for binding.
    pub view: wgpu::TextureView,
    /// Bind group for this icon.
    pub bind_group: wgpu::BindGroup,
    /// Icon width.
    pub width: u32,
    /// Icon height.
    pub height: u32,
}

/// Icon renderer for SVG icons.
pub struct IconRenderer {
    /// Render pipeline for textured quads.
    pipeline: wgpu::RenderPipeline,
    /// Bind group layout for textures.
    bind_group_layout: wgpu::BindGroupLayout,
    /// Vertex buffer.
    vertex_buffer: wgpu::Buffer,
    /// Texture sampler.
    sampler: wgpu::Sampler,
    /// Cached icon textures by name.
    icon_cache: HashMap<String, IconTexture>,
}

impl IconRenderer {
    /// Create a new icon renderer.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Icon Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("icon.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Icon Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Icon Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Icon Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[TexturedVertex::desc()],
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

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Icon Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Pre-allocate buffer for 100 icons
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Icon Vertex Buffer"),
            size: (100 * 6 * std::mem::size_of::<TexturedVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            bind_group_layout,
            vertex_buffer,
            sampler,
            icon_cache: HashMap::new(),
        }
    }

    /// Load an SVG icon and cache it as a GPU texture.
    pub fn load_icon(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        name: &str,
        svg_data: &str,
        target_size: u32,
    ) -> bool {
        if self.icon_cache.contains_key(name) {
            return true;
        }

        let Some(rasterized) = RasterizedIcon::from_svg(svg_data, target_size) else {
            return false;
        };

        let bytes_per_row = rasterized.width * 4;
        let padded_bytes_per_row = (bytes_per_row + 255) & !255;

        // If padding is needed, we must copy data to a new buffer
        let data = if bytes_per_row == padded_bytes_per_row {
            std::borrow::Cow::Borrowed(&rasterized.pixels)
        } else {
            let mut padded_data =
                Vec::with_capacity((padded_bytes_per_row * rasterized.height) as usize);
            for row in 0..rasterized.height {
                let start = (row * bytes_per_row) as usize;
                let end = start + bytes_per_row as usize;
                padded_data.extend_from_slice(&rasterized.pixels[start..end]);
                // Add padding
                padded_data.extend(std::iter::repeat_n(
                    0,
                    (padded_bytes_per_row - bytes_per_row) as usize,
                ));
            }
            std::borrow::Cow::Owned(padded_data)
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("Icon Texture: {}", name)),
            size: wgpu::Extent3d {
                width: rasterized.width,
                height: rasterized.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(rasterized.height),
            },
            wgpu::Extent3d {
                width: rasterized.width,
                height: rasterized.height,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("Icon Bind Group: {}", name)),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        self.icon_cache.insert(
            name.to_string(),
            IconTexture {
                texture,
                view,
                bind_group,
                width: rasterized.width,
                height: rasterized.height,
            },
        );

        true
    }

    /// Check if an icon is loaded.
    pub fn has_icon(&self, name: &str) -> bool {
        self.icon_cache.contains_key(name)
    }

    /// Render an icon at the specified position.
    #[allow(clippy::too_many_arguments)]
    pub fn render_icon(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        queue: &wgpu::Queue,
        icon_name: &str,
        x: f32,
        y: f32,
        size: f32,
        screen_width: f32,
        screen_height: f32,
        tint: [f32; 4],
    ) {
        let Some(icon) = self.icon_cache.get(icon_name) else {
            return;
        };

        // Generate vertices for a textured quad
        let vertices =
            self.create_quad_vertices(x, y, size, size, screen_width, screen_height, tint);
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Icon Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &icon.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }

    /// Render multiple icons in a single batch.
    #[allow(clippy::too_many_arguments)]
    pub fn render_icons(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        queue: &wgpu::Queue,
        icons: &[(&str, f32, f32, f32, [f32; 4])], // (name, x, y, size, tint)
        screen_width: f32,
        screen_height: f32,
    ) {
        for (name, x, y, size, tint) in icons {
            self.render_icon(
                encoder,
                view,
                queue,
                name,
                *x,
                *y,
                *size,
                screen_width,
                screen_height,
                *tint,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_quad_vertices(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        screen_width: f32,
        screen_height: f32,
        tint: [f32; 4],
    ) -> [TexturedVertex; 6] {
        // Convert pixel coordinates to NDC (-1 to 1)
        let x1 = (x / screen_width) * 2.0 - 1.0;
        let y1 = 1.0 - (y / screen_height) * 2.0;
        let x2 = ((x + width) / screen_width) * 2.0 - 1.0;
        let y2 = 1.0 - ((y + height) / screen_height) * 2.0;

        [
            TexturedVertex {
                position: [x1, y1],
                tex_coords: [0.0, 0.0],
                color: tint,
            },
            TexturedVertex {
                position: [x2, y1],
                tex_coords: [1.0, 0.0],
                color: tint,
            },
            TexturedVertex {
                position: [x1, y2],
                tex_coords: [0.0, 1.0],
                color: tint,
            },
            TexturedVertex {
                position: [x1, y2],
                tex_coords: [0.0, 1.0],
                color: tint,
            },
            TexturedVertex {
                position: [x2, y1],
                tex_coords: [1.0, 0.0],
                color: tint,
            },
            TexturedVertex {
                position: [x2, y2],
                tex_coords: [1.0, 1.0],
                color: tint,
            },
        ]
    }
}
