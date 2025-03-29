// src/lib.rs

pub mod renderer;
pub mod ecs;
pub mod input;
pub mod game;
pub mod config; // 追加

// ルートから直接アクセスできるように再エクスポート
pub use renderer::Renderer;
pub use ecs::World;
pub use input::InputState;
pub use game::{run_game, Game};
pub use config::GameConfig;

