// 必要なモジュールをインポートします。
// 利用者側は run_game 関数と Game トレイトを実装するだけです。
use mothra::{run_game, World, Renderer, InputState, Game};
use std::time::{Duration, Instant};

/// テスト用のゲームロジックを実装する構造体。
/// 更新回数が100回に達したらプログラムを終了します。
struct TestGame {
    update_count: u32,
    start_time: Instant,
}

impl TestGame {
    /// TestGame の新しいインスタンスを生成する。
    fn new() -> Self {
        Self {
            update_count: 0,
            start_time: Instant::now(),
        }
    }
}

/// Game トレイトの実装。
impl Game for TestGame {
    /// 毎フレームの更新処理。
    /// ここでは更新回数をカウントし、一定回数に達したら終了します。
    fn update(&mut self, _world: &mut World, _renderer: &mut Renderer, _input: &InputState) {
        self.update_count += 1;
        println!("Update count: {}", self.update_count);
        // 更新回数が100回に達したら、テスト用にアプリケーションを終了する
        if self.update_count >= 100 {
            // 終了処理（テスト用）
            std::process::exit(0);
        }
    }

    /// 毎フレームの描画処理。
    /// 内部で Renderer の描画メソッドを呼び出し、World の状態に基づいた描画を行います。
    fn render(&mut self, world: &World, renderer: &mut Renderer) {
        // サーフェスからフレームを取得し、レンダーパスを開始する
        let output = renderer.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        
        // Renderer の draw_world() を呼び出して、World のエンティティを描画する
        renderer.draw_world(&mut encoder, &view, world);
        
        renderer.queue.submit(Some(encoder.finish()));
        output.present();
    }
}

/// エントリーポイント。run_game() 関数に TestGame インスタンスを渡すだけで起動できます。
fn main() {
    run_game(TestGame::new());
}
