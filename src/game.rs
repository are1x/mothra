use crate::ecs::World;
use crate::input::InputState;
use crate::renderer::Renderer;
use crate::config::GameConfig;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;
use std::time::{Duration, Instant};

/// ゲームのメインロジックを定義するトレイト。
pub trait Game {
    /// 毎フレームの更新処理。
    fn update(&mut self, world: &mut World, renderer: &mut Renderer, input: &InputState);
    /// 毎フレームの描画処理。`view` と `encoder` を使って描画コマンドを記録する。
    fn render(&mut self, world: &World, renderer: &mut Renderer, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder);
}

/// run_game 関数
///
/// この関数は、Gameトレイトを実装したゲームロジックと設定情報(GameConfig)を受け取り、
/// 内部でウィンドウ生成、Renderer、World、InputState の初期化、FPS制御付きイベントループを管理します。
pub fn run_game<G: 'static + Game>(mut game: G, config: GameConfig) -> ! {
    // イベントループとウィンドウの作成
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.title.clone())
        .with_inner_size(winit::dpi::LogicalSize::new(config.window_width, config.window_height))
        .build(&event_loop)
        .unwrap();

    // Renderer, World, InputState の初期化
    let mut renderer = block_on(Renderer::new(&window));
    let mut world = crate::ecs::World::new();
    let mut input = crate::input::InputState::default();

    // FPS制御用の目標フレーム時間
    let target_frame_duration = Duration::from_millis(1000 / config.target_fps as u64);
    let mut last_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { ref event, .. } => {
                input.update(event);
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                let elapsed = now - last_frame_time;
                if elapsed < target_frame_duration {
                    *control_flow = ControlFlow::WaitUntil(now + target_frame_duration - elapsed);
                } else {
                    last_frame_time = now;
                    game.update(&mut world, &mut renderer, &input);
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                let output = match renderer.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(_) => {
                        renderer.surface.configure(&renderer.device, &renderer.config);
                        renderer.surface
                            .get_current_texture()
                            .expect("Failed to acquire texture")
                    }
                };
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                game.render(&world, &mut renderer, &view, &mut encoder);
                renderer.queue.submit(Some(encoder.finish()));
                output.present();
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
