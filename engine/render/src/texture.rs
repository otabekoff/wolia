//! Texture atlas management.

use wolia_math::Size;

use crate::context::RenderContext;

/// A texture atlas for efficient GPU texture management.
pub struct TextureAtlas {
    /// The GPU texture.
    pub texture: wgpu::Texture,
    /// Texture size.
    pub size: Size,
    /// Texture view.
    pub view: wgpu::TextureView,
}

impl TextureAtlas {
    /// Create a new texture atlas.
    pub fn new(context: &RenderContext, size: Size) -> Self {
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Wolia Texture Atlas"),
            size: wgpu::Extent3d {
                width: size.width as u32,
                height: size.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            size,
            view,
        }
    }

    /// Upload image data to the atlas.
    pub fn upload(
        &self,
        context: &RenderContext,
        data: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) {
        context.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }
}
