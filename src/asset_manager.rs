// src/asset_manager.rs

use std::collections::HashMap;
use std::rc::Rc;
use wgpu::{Device, ShaderModule};
use crate::renderer::TextureHandle;

/// アセット管理用の構造体。
/// テクスチャとシェーダーをキャッシュして、重複読み込みを防ぎます。
pub struct AssetManager {
    pub textures: HashMap<String, Rc<TextureHandle>>,
    pub shaders: HashMap<String, Rc<ShaderModule>>,
}

impl AssetManager {
    /// 新しい AssetManager を生成する
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    /// 指定されたパスのテクスチャをキャッシュから取得、もしくは新たに読み込みます。
    pub fn load_texture(&mut self, device: &Device, queue: &wgpu::Queue, path: &str) -> Rc<TextureHandle> {
        if let Some(texture) = self.textures.get(path) {
            return Rc::clone(texture);
        }
        // 画像読み込み処理
        let img = image::open(path).expect("Failed to open image").to_rgba8();
        let (width, height) = img.dimensions();
        let size = wgpu::Extent3d { width, height, depth_or_array_layers: 1 };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("User Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let texture_handle = TextureHandle { texture, view, sampler };
        let rc_handle = Rc::new(texture_handle);
        self.textures.insert(path.to_string(), Rc::clone(&rc_handle));
        rc_handle
    }

    /// 指定されたパスのシェーダーをキャッシュから取得、もしくは新たに読み込みます。
    pub fn load_shader(&mut self, device: &Device, path: &str) -> Rc<ShaderModule> {
        if let Some(shader) = self.shaders.get(path) {
            return Rc::clone(shader);
        }
        let shader_src = std::fs::read_to_string(path).expect("Failed to read shader file");
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(path),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });
        let rc_shader = Rc::new(shader_module);
        self.shaders.insert(path.to_string(), Rc::clone(&rc_shader));
        rc_shader
    }
}
