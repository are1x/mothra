// src/renderer.rs

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::ecs::World;
use crate::GameConfig;

/// 描画エンジンの中心構造体。WGPU の初期化、描画処理、リソース管理などを担当する。
pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub surface_format: wgpu::TextureFormat,

    // テクスチャ描画用のリソース（シェーダー、パイプラインなど）
    texture_pipeline: wgpu::RenderPipeline,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    // ユニフォーム用のバッファとバインドグループ
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    // 追加：テクスチャ用 bind group のキャッシュ
    texture_bind_group_cache: RefCell<HashMap<*const TextureHandle, Rc<wgpu::BindGroup>>>,
}

/// テクスチャとサンプラーをまとめた構造体。
/// テクスチャ本体も保持することで、ビューが無効にならないようにする。
pub struct TextureHandle {
    pub texture: wgpu::Texture,  // 追加: テクスチャ本体を保持
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}


impl Renderer {
    /// Renderer構造体の初期化。
    /// ウィンドウと連携し、WGPUの初期化・パイプライン・バインドレイアウトをセットアップする。
    pub async fn new(window: &Window) -> Self {
        use wgpu::util::DeviceExt;

        // ウィンドウサイズ取得（物理サイズ）
        let size = window.inner_size();

        // 固定の論理サイズ
        let logical_width: f32 = 800.0;
        let logical_height: f32 = 600.0;

        // WGPUインスタンスとサーフェス作成
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(window) }.unwrap();

        // アダプター取得
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await.unwrap();

        // デバイスとキューの作成
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        // サーフェスのフォーマットと設定
        let surface_format = surface.get_capabilities(&adapter).formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![surface_format],
        };
        surface.configure(&device, &config);

        // バインドグループレイアウト（group 0: uniforms）
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform BindGroup Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // 固定の論理サイズから uniform のスケール値を計算
        // (論理座標 (0,0)-(800,600) を NDC (-1,-1)-(1,1) に変換する)
        let uniform_data: [f32; 2] = [2.0 / logical_width, 2.0 / logical_height];
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&uniform_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform BindGroup"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // 次に、group 1: texture + sampler のレイアウトを作成
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture BindGroup Layout"),
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

        // シェーダー読み込み
        let shader_src = std::fs::read_to_string("assets/shader_texture.wgsl").unwrap();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Texture Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });

        // パイプラインレイアウト（2つのbind group layoutを指定）
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Texture Pipeline Layout"),
            bind_group_layouts: &[
                &uniform_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        // テクスチャ描画用のパイプライン作成
        let texture_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Texture Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // インデックスバッファ（四角形）
        let index_data: [u16; 6] = [0, 1, 2, 2, 3, 0];
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        // ダミー頂点バッファ（必要に応じて draw 時に書き換える）
        let vertex_data: [[f32; 4]; 4] = [[0.0; 4]; 4];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // 構造体の生成・返却
        Self {
            device,
            queue,
            surface,
            config,
            surface_format,
            texture_pipeline,
            texture_bind_group_layout,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            uniform_bind_group,
            texture_bind_group_cache: RefCell::new(HashMap::new()),
        }
    }

    /// 指定したスケール値で uniform_buffer を更新します。
    pub fn update_uniform(&self, scale: &[f32; 2]) {
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(scale));
    }

    /// ウィンドウサイズが変更されたときの処理。
    /// 新しい物理サイズでサーフェスを再構成し、stretch_mode に応じて uniform_buffer を更新する。
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, config: &GameConfig) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            // stretch_mode の値によって、uniform のスケールを決定する
            let scale = if config.stretch_mode {
                // ウィンドウの物理サイズに合わせる
                [2.0 / new_size.width as f32, 2.0 / new_size.height as f32]
            } else {
                // 論理解像度を固定（config.logical_width, config.logical_height に基づく）
                [2.0 / config.logical_width as f32, 2.0 / config.logical_height as f32]
            };
            self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&scale));
        }
    }

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            Ok(tex) => tex,
            Err(_) => {
                self.surface.configure(&self.device, &self.config);
                self.surface.get_current_texture().expect("Surface再取得失敗")
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            // 他の描画処理（draw_quad, draw_texture など）を呼び出す
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
    }

    /// テクスチャを読み込み、GPUへ転送して TextureHandle を返す。
    /// 
    /// # 引数
    /// * `path` - 画像ファイルのパス
    ///
    /// # 戻り値
    /// * `TextureHandle` - view + sampler を含む構造体
    pub fn load_texture(&self, path: &str) -> TextureHandle {
        use image::GenericImageView;
    
        let img = image::open(path).expect("Failed to open image").to_rgba8();
        let (width, height) = img.dimensions();
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
    
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("User Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
    
        self.queue.write_texture(
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
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
    
        TextureHandle {
            texture, // テクスチャ本体を保持する
            view,
            sampler,
        }
    }
    

    /// 指定したテクスチャを、指定した領域に描画する。
    ///
    /// # 引数
    /// * `encoder` - コマンドエンコーダ
    /// * `view` - 描画対象のテクスチャビュー
    /// * `texture` - 描画対象のテクスチャ（ハンドル）
    /// * `x`, `y`, `w`, `h` - 描画する矩形の左下座標とサイズ（論理座標）
    pub fn draw_texture(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        texture: &TextureHandle,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        // ここでは論理座標系（0,0)-(800,600) を前提とするので、
        // 頂点データはそのまま論理座標で渡す
        let vertex_data = [
            [x, y + h, 0.0, 0.0],     // 左上
            [x + w, y + h, 1.0, 0.0],   // 右上
            [x + w, y, 1.0, 1.0],       // 右下
            [x, y, 0.0, 1.0],           // 左下
        ];
    
        self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertex_data));
    
        // テクスチャ用 bind group を作成（group 1）
        let texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("Texture BindGroup"),
        });
    
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Texture Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    
        // パイプラインを最初にセットする
        pass.set_pipeline(&self.texture_pipeline);
    
        // シェーダーのバインド順に合わせる
        pass.set_bind_group(0, &self.uniform_bind_group, &[]); // ユニフォーム（group 0）
        pass.set_bind_group(1, &texture_bind_group, &[]);        // テクスチャ＋サンプラー（group 1）
    
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..6, 0, 0..1);
    }

    /// World 内のエンティティをすべて描画する。
    /// ここでは、各エンティティごとに新しい bind group を作成し、ローカルなベクターに保持してから描画します。
    pub fn draw_world(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        world: &crate::ecs::World,
    ) {
        // 各エンティティごとのリソースを保持するベクターを用意する
        let mut entity_vertex_buffers: Vec<wgpu::Buffer> = Vec::new();
        let mut entity_bind_groups: Vec<wgpu::BindGroup> = Vec::new();
        let mut transforms: Vec<crate::ecs::Transform> = Vec::new();
    
        // すべての描画対象エンティティについて、各リソースを生成して保持する
        for (transform, texture) in world.query_drawables() {
            transforms.push(transform);
            // 論理座標系 (0,0)-(800,600) を前提とする頂点データ
            let vertex_data = [
                [transform.x, transform.y + transform.h, 0.0, 0.0],     // 左上
                [transform.x + transform.w, transform.y + transform.h, 1.0, 0.0], // 右上
                [transform.x + transform.w, transform.y, 1.0, 1.0],       // 右下
                [transform.x, transform.y, 0.0, 1.0],                     // 左下
            ];
            let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Entity Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            });
            entity_vertex_buffers.push(vb);
    
            let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
                ],
                label: Some("Entity Texture BindGroup"),
            });
            entity_bind_groups.push(bg);
        }
    
        // レンダーパスを一度だけ開始する
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("World Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
    
            pass.set_pipeline(&self.texture_pipeline);
            // ユニフォームは共通
            pass.set_bind_group(0, &self.uniform_bind_group, &[]);
    
            // 各エンティティごとに描画コマンドを記録する
            for (i, _transform) in transforms.iter().enumerate() {
                pass.set_bind_group(1, &entity_bind_groups[i], &[]);
                pass.set_vertex_buffer(0, entity_vertex_buffers[i].slice(..));
                pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..6, 0, 0..1);
            }
        }
        // レンダーパス終了後、上記ベクターに保持していたリソースは drop されますが、
        // コマンドバッファには既に記録されているので問題ありません。
    }
    
    

}
