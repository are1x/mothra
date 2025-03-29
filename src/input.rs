// 入力イベントを表す列挙型
#[derive(Debug)]
pub enum InputEvent {
    KeyPressed(winit::event::VirtualKeyCode),
    KeyReleased(winit::event::VirtualKeyCode),
    MouseMoved { x: f64, y: f64 },
    MouseButtonPressed(winit::event::MouseButton),
    MouseButtonReleased(winit::event::MouseButton),
    // 将来的にタッチ入力なども追加可能
}

/// 入力状態を保持する構造体
#[derive(Default, Debug)]
pub struct InputState {
    /// 現在押されているキー
    pub keys_pressed: Vec<winit::event::VirtualKeyCode>,
    /// 現在押されているマウスボタン
    pub mouse_buttons: Vec<winit::event::MouseButton>,
    /// カーソルの現在位置（ウィンドウ座標）
    pub cursor_position: Option<(f64, f64)>,
}

impl InputState {
    /// 与えられたウィンドウイベントに基づき入力状態を更新する
    pub fn update(&mut self, event: &winit::event::WindowEvent) {
        use winit::event::{ElementState, MouseButton, VirtualKeyCode, WindowEvent};

        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            if !self.keys_pressed.contains(&key) {
                                self.keys_pressed.push(key);
                            }
                        }
                        ElementState::Released => {
                            self.keys_pressed.retain(|&k| k != key);
                        }
                    }
                }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                match state {
                    ElementState::Pressed => {
                        if !self.mouse_buttons.contains(button) {
                            self.mouse_buttons.push(*button);
                        }
                    }
                    ElementState::Released => {
                        self.mouse_buttons.retain(|&b| b != *button);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = Some((position.x, position.y));
            }
            _ => {}
        }
    }
}

/// 入力イベントをディスパッチする機能を持つ構造体
pub struct InputDispatcher {
    events: Vec<InputEvent>,
}

impl InputDispatcher {
    /// 新しいディスパッチャを生成する
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// ウィンドウイベントから入力イベントを生成して内部キューに追加する
    pub fn dispatch(&mut self, event: &winit::event::WindowEvent) {
        use winit::event::{ElementState, MouseButton, VirtualKeyCode, WindowEvent};

        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.events.push(InputEvent::KeyPressed(key));
                        }
                        ElementState::Released => {
                            self.events.push(InputEvent::KeyReleased(key));
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.events.push(InputEvent::MouseMoved {
                    x: position.x,
                    y: position.y,
                });
            }
            WindowEvent::MouseInput { button, state, .. } => {
                match state {
                    ElementState::Pressed => {
                        self.events.push(InputEvent::MouseButtonPressed(*button));
                    }
                    ElementState::Released => {
                        self.events.push(InputEvent::MouseButtonReleased(*button));
                    }
                }
            }
            _ => {}
        }
    }

    /// 内部キューにたまったイベントをすべて取り出し、返す
    pub fn drain(&mut self) -> Vec<InputEvent> {
        self.events.drain(..).collect()
    }
}
