use mothra::{run_game, Game, World, Renderer, InputState, GameConfig, AssetManager};
use std::rc::Rc;
use std::time::Instant;

/// TestAssetGame は、AssetManager のキャッシュ機能と ECS を使ったテクスチャ表示をテストするためのゲームロジックです。
struct TestAssetGame {
    update_count: u32,
    start_time: Instant,
    asset_manager: AssetManager,
}

impl TestAssetGame {
    /// TestAssetGame の新しいインスタンスを生成する
    fn new() -> Self {
        Self {
            update_count: 0,
            start_time: Instant::now(),
            asset_manager: AssetManager::new(),
        }
    }
}

impl Game for TestAssetGame {
    /// 更新処理：初回に AssetManager を使って同じ画像と異なる画像を読み込み、キャッシュ機能をテストし、
    /// それぞれのテクスチャでエンティティを生成します。
    fn update(&mut self, world: &mut World, renderer: &mut Renderer, _input: &InputState) {
        self.update_count += 1;
        if self.update_count == 1 {
            // 同じ画像を2回読み込む
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
            // 異なる画像
            let tex_white = self.asset_manager.load_texture(
                &renderer.device,
                &renderer.queue,
                "assets/textures/white_plane_image.png",
            );
            
            // キャッシュ機能の確認
            println!(
                "tex_black1 and tex_black2 are the same: {}",
                Rc::ptr_eq(&tex_black1, &tex_black2)
            );
            println!(
                "tex_black1 and tex_white are the same: {}",
                Rc::ptr_eq(&tex_black1, &tex_white)
            );
            
            // 1つ目のエンティティ：黒いテクスチャ
            let e1 = world.spawn();
            world.add_transform(
                e1,
                mothra::ecs::Transform {
                    x: 100.0,
                    y: 150.0,
                    w: 128.0,
                    h: 128.0,
                },
            );
            world.add_sprite(
                e1,
                mothra::ecs::Sprite {
                    texture: Rc::clone(&tex_black1),
                },
            );
            
            // 2つ目のエンティティ：白いテクスチャ
            let e2 = world.spawn();
            world.add_transform(
                e2,
                mothra::ecs::Transform {
                    x: 300.0,
                    y: 150.0,
                    w: 128.0,
                    h: 128.0,
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

    /// 描画処理：Renderer の draw_world() を呼び出して、World に登録されたエンティティを描画します。
    fn render(&mut self, world: &World, renderer: &mut Renderer, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        renderer.draw_world(encoder, view, world);
    }
}

/// エントリーポイント：GameConfig でウィンドウ設定やFPSを指定して run_game を呼び出す。
fn main() {
    let config = GameConfig {
        window_width: 800,
        window_height: 600,
        logical_width: 800,
        logical_height:600,
        title: "Test Asset Manager".to_string(),
        target_fps: 60,
        stretch_mode:false
    };

    // run_game 内部で、ウィンドウ生成、Renderer、World、InputState の初期化およびイベントループが管理されます。
    run_game(TestAssetGame::new(), config);
}
