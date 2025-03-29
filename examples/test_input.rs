use winit::{
    event::{Event, WindowEvent, ElementState, VirtualKeyCode, MouseButton},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;
use mothra::input::{InputDispatcher, InputState, InputEvent};

fn main() {
    // イベントループとウィンドウの作成
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Test Input Module")
        .build(&event_loop)
        .unwrap();

    // 入力状態とディスパッチャの初期化
    let mut input_state = InputState::default();
    let mut input_dispatcher = InputDispatcher::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match &event {
            // ウィンドウイベントを受け取ったら、入力状態の更新とイベントのディスパッチを行う
            Event::WindowEvent { event, .. } => {
                input_state.update(event);
                input_dispatcher.dispatch(event);
            }
            Event::MainEventsCleared => {
                // 毎フレーム描画前に再描画リクエスト
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // 現在の入力状態を表示
                //println!("Current Input State: {:?}", input_state);
                // 蓄積された入力イベントを取り出して表示
                let events = input_dispatcher.drain();
                if !events.is_empty() {
                    println!("Dispatched events:");
                    for e in events {
                        println!("  {:?}", e);
                    }
                }
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
