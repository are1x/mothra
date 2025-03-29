use mothra::{run_game, Game, World, Renderer, InputState, GameConfig, AssetManager};
use std::rc::Rc;
use std::time::Instant;

/// TestSpriteGame は、AssetManager のキャッシュ機能と
/// ECS を使ったスプライト描画の性能比較テストを行うためのゲームロジックです。
struct TestSpriteGame {
    update_count: u32,
    start_time: Instant,
    asset_manager: AssetManager,
}

impl TestSpriteGame {
    /// TestSpriteGame の新しいインスタンスを生成する
    fn new() -> Self {
        Self {
            update_count: 0,
            start_time: Instant::now(),
            asset_manager: AssetManager::new(),
        }
    }
}

impl Game for TestSpriteGame {
    /// 毎フレームの更新処理。
    /// 初回の更新時に AssetManager を使って同じテクスチャ（黒い画像）と異なるテクスチャ（白い画像）を読み込み、
    /// キャッシュが機能しているかを確認しつつ、World にエンティティを追加します。
    fn update(&mut self, world: &mut World, renderer: &mut Renderer, _input: &InputState) {
        self.update_count += 1;
        if self.update_count == 1 {
            // 同じテクスチャを2回読み込んでキャッシュを検証
            let tex_black1 = self.asset_manager.load_texture(
                &renderer.device,
                &renderer.queue,
                "assets/textures/black_plane_image.png",
            );
            let tex_black2 = self.asset_manager.load_texture(
                &renderer.device,
                &renderer.queue,
                "assets/textures/black_plane_image.png",
            );
            let tex_white = self.asset_manager.load_texture(
                &renderer.device,
                &renderer.queue,
                "assets/textures/white_plane_image.png",
            );

            println!(
                "tex_black1 and tex_black2 are the same: {}",
                Rc::ptr_eq(&tex_black1, &tex_black2)
            );
            println!(
                "tex_black1 and tex_white are the same: {}",
                Rc::ptr_eq(&tex_black1, &tex_white)
            );

            // エンティティを生成して World に追加
            let e1 = world.spawn();
            world.add_transform(
                e1,
                mothra::ecs::Transform {
                    x: 100.0,
                    y: 150.0,
                    w: 128.0,
                    h: 128.0,
                    z: 0.5, // z値で奥行きを指定
                },
            );
            world.add_sprite(
                e1,
                mothra::ecs::Sprite {
                    texture: Rc::clone(&tex_black1),
                },
            );

            let e2 = world.spawn();
            world.add_transform(
                e2,
                mothra::ecs::Transform {
                    x: 300.0,
                    y: 150.0,
                    w: 128.0,
                    h: 128.0,
                    z: 1.0,
                },
            );
            world.add_sprite(
                e2,
                mothra::ecs::Sprite {
                    texture: Rc::clone(&tex_white),
                },
            );
        }
    }

    /// 毎フレームの描画処理。
    /// update_count の偶数フレームでは従来の個別描画（draw_world）を、
    /// 奇数フレームでは新しいバッチ描画（draw_sprites_batched）を呼び出し、
    /// それぞれの処理時間をコンソールに出力します。
    fn render(&mut self, world: &World, renderer: &mut Renderer, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let start = Instant::now();
        if self.update_count % 2 == 0 {
            // 従来の描画（各エンティティ毎に個別に描画）
            renderer.draw_world(encoder, view, world);
            let elapsed = start.elapsed();
            println!("draw_world elapsed: {:?}", elapsed);
        } else {
            // 新しいバッチ描画（同一テクスチャをグループ化して一括描画）
            renderer.draw_sprites_batched(encoder, view, world);
            let elapsed = start.elapsed();
            println!("draw_sprites_batched elapsed: {:?}", elapsed);
        }
    }
}

fn main() {
    let config = GameConfig {
        window_width: 800,
        window_height: 600,
        logical_width: 800,
        logical_height: 600,
        title: "Test Sprite Batch vs Individual".to_string(),
        target_fps: 60,
        stretch_mode: false,
    };

    run_game(TestSpriteGame::new(), config);
}
