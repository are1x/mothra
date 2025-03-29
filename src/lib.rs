// src/lib.rs
pub mod renderer;
pub mod ecs;
pub mod input;
pub mod game;
pub mod config;
pub mod asset_manager;
pub mod logger;


pub use renderer::Renderer;
pub use ecs::World;
pub use input::InputState;
pub use game::{run_game, Game};
pub use config::GameConfig;
pub use asset_manager::AssetManager;
pub use logger::init_logger_with_config;
