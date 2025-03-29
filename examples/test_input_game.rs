use mothra::{run_game, Game, World, Renderer, InputState, GameConfig};
use std::time::Instant;

/// テスト用ゲームロジック
///
/// このゲームは、毎フレーム update() でカウンタを更新し、
/// render() 内で Renderer の draw_world() を呼び出して ECS の内容を描画します。
struct TestGame {
    update_count: u32,
    start_time: Instant,
}

impl TestGame {
    /// TestGame の新しいインスタンスを生成する
    fn new() -> Self {
        Self {
            update_count: 0,
            start_time: Instant::now(),
        }
    }
}

impl Game for TestGame {
    /// 毎フレームの更新処理。ここでは単純にカウンタを更新し、60フレームごとにログ出力します。
    fn update(&mut self, _world: &mut World, _renderer: &mut Renderer, _input: &InputState) {
        self.update_count += 1;
        if self.update_count % 60 == 0 {
            println!("Update count: {}", self.update_count);
        }
    }

    /// 毎フレームの描画処理。内部で Renderer の draw_world() を呼び出して、World に登録されたエンティティを描画します。
    fn render(&mut self, world: &World, renderer: &mut Renderer, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        renderer.draw_world(encoder, view, world);
    }
}

fn main() {
    // GameConfig によりウィンドウサイズ、タイトル、FPS などを設定
    let config = GameConfig {
        window_width: 800,
        window_height: 600,
        title: "Test Input Game".to_string(),
        target_fps: 60,
    };

    // run_game に TestGame インスタンスと設定を渡すだけで起動
    run_game(TestGame::new(), config);
}
