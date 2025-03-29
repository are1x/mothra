// src/lib.rs

pub mod renderer;
pub mod ecs;
pub mod input;
pub mod game;

// ルートから直接アクセスできるように再エクスポートする
pub use renderer::Renderer;
pub use ecs::World;
pub use input::InputState;
pub use game::{run_game, Game};
