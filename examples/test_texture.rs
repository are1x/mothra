use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use mothra::renderer::Renderer; // ← `lib.rs` 経由で `renderer.rs` にある `Renderer` を呼び出す

fn main() {
    
    // イベントループとウィンドウ作成
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Test Texture Rendering")
        .build(&event_loop)
        .unwrap();

    // レンダラー初期化（非同期ブロッキング）
    let mut renderer = pollster::block_on(Renderer::new(&window));
    let tex = renderer.load_texture("assets/textures/black_plane_image.png");

    // 今は描画する内容がないので、Renderer::render() が画面クリアだけ行う状態でもOK

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                renderer.resize(new_size);
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                let output = renderer.surface.get_current_texture().unwrap();
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                renderer.draw_texture(&mut encoder, &view, &tex, 100.0, 100.0, 256.0, 256.0);

                renderer.queue.submit(Some(encoder.finish()));
                output.present();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            _ => {}
        }
    });
}
