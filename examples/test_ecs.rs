use mothra::ecs::{Sprite, Transform, World};
use mothra::renderer::Renderer;
use pollster::block_on;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::rc::Rc;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Test ECS Multiple Entities")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let mut renderer = block_on(Renderer::new(&window));

    // テクスチャ読み込み（すべてのエンティティで使い回す）
    let texture = Rc::new(renderer.load_texture("assets/textures/black_plane_image.png"));

    // World を構築
    let mut world = World::new();

    // 複数 Entity を生成
    for i in 0..5 {
        let entity = world.spawn();
        world.add_transform(
            entity,
            Transform {
                x: 50.0 + i as f32 * 150.0,
                y: 200.0,
                w: 128.0,
                h: 128.0,
            },
        );
        world.add_sprite(
            entity,
            Sprite {
                texture: Rc::clone(&texture),
            },
        );
    }

    // メインループ
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                let output = match renderer.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(_) => {
                        renderer.surface.configure(&renderer.device, &renderer.config);
                        renderer
                            .surface
                            .get_current_texture()
                            .expect("Surface再取得失敗")
                    }
                };

                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                // draw_world を呼び出して、1つのレンダーパス内で全エンティティを描画する
                renderer.draw_world(&mut encoder, &view, &world);
                
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
