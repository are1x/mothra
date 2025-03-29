use mothra::{run_game, Game, World, Renderer, InputState, GameConfig, AssetManager};
use std::rc::Rc;
use std::time::Instant;

/// TestAssetGame は Game トレイトを実装し、AssetManager の機能をテストするためのゲームロジックです。
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
    /// 毎フレームの更新処理。
    /// 初回の更新時に、同じテクスチャを2回と別のテクスチャを1回読み込み、キャッシュ動作を確認し、
    /// それらを用いて2つのエンティティを World に追加します。
    fn update(&mut self, world: &mut World, renderer: &mut Renderer, _input: &InputState) {
        self.update_count += 1;
        if self.update_count == 1 {
            // AssetManager を使ってテクスチャを読み込む
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

            // エンティティを生成して、テクスチャを設定する
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

    /// 毎フレームの描画処理。
    /// Renderer の draw_world() を呼び出して、World に登録されたエンティティを描画します。
    fn render(&mut self, world: &World, renderer: &mut Renderer, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        renderer.draw_world(encoder, view, world);
    }
}

/// エントリーポイント：GameConfig によりウィンドウ設定やFPSを指定して run_game を呼び出す。
fn main() {
    let config = GameConfig {
        window_width: 800,
        window_height: 600,
        title: "Test Asset Game".to_string(),
        target_fps: 60,
    };

    // run_game 内部でイベントループが起動し、ウィンドウが閉じられるまで実行されます。
    run_game(TestAssetGame::new(), config);
}
