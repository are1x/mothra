
use crate::World;
use crate::Renderer;
use crate::InputState;
/// ゲームのメインロジックを定義するトレイト。

pub trait Game {
    /// 毎フレームの更新処理。World やレンダラー、入力状態を受け取り、ゲーム状態を更新する。
    fn update(&mut self, world: &mut World, renderer: &mut Renderer, input: &InputState);

    /// 毎フレームの描画処理。更新済みの状態を元にレンダリングを行う。
    fn render(&mut self, world: &World, renderer: &mut Renderer);
}

/// ゲームエンジンを起動するエントリーポイント関数。
///
/// 利用者はこの関数に自分のゲームロジックを実装した型を渡すだけで、
/// ウィンドウ生成、レンダラー、ECSのWorld、入力処理などが内部で初期化され、
/// イベントループが自動で実行されます。
///
/// # 引数
/// * `game` - ゲームロジックを実装したオブジェクト（Gameトレイトの実装）
///
/// # 例
/// ```rust
/// struct MyGame { /* 独自の状態 */ }
///
/// impl Game for MyGame {
///     fn update(&mut self, world: &mut World, renderer: &mut Renderer, input: &InputState) {
///         // ゲームの更新処理
///     }
///
///     fn render(&mut self, world: &World, renderer: &mut Renderer) {
///         // 描画処理
///     }
/// }
///
/// fn main() {
///     mothra::run_game(MyGame { /* 初期状態 */ });
/// }
/// ```
pub fn run_game<G: 'static + Game>(mut game: G) -> ! {
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    // ウィンドウ作成
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mothra Engine")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    // Renderer, World, InputState の初期化（各自のモジュールで定義済みと仮定）
    let mut renderer = pollster::block_on(Renderer::new(&window));
    let mut world = World::new();
    let mut input = InputState::default();

    // （ここで初期シーン構築や入力リソースの設定も行う）

    // イベントループ
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { ref event, .. } => {
                // 入力状態更新（input.rs 内の実装を利用）
                input.update(event);
            }
            Event::MainEventsCleared => {
                // ゲームロジックの更新
                game.update(&mut world, &mut renderer, &input);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // ゲームロジックの描画
                game.render(&world, &mut renderer);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
