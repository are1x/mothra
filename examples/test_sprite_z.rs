use mothra::{logger, run_game, AssetManager, Game, GameConfig, InputState, Renderer, World};
use std::rc::Rc;
use std::time::Instant;

struct TestSpriteZ {
    update_count: u32,
    start_time: Instant,
    asset_manager: AssetManager,
}

impl TestSpriteZ {
    fn new() -> Self {
        Self {
            update_count: 0,
            start_time: Instant::now(),
            asset_manager: AssetManager::new(),
        }
    }
}

impl Game for TestSpriteZ {
    fn update(&mut self, world: &mut World, renderer: &mut Renderer, _input: &InputState) {
        self.update_count += 1;
        if self.update_count == 1 {
            let tex_black = self.asset_manager.load_texture(
                &renderer.device,
                &renderer.queue,
                "assets/textures/black_plane_image.png",
            );
            let tex_white = self.asset_manager.load_texture(
                &renderer.device,
                &renderer.queue,
                "assets/textures/white_plane_image.png",
            );
            let entities = vec![
                (100.0, 100.0, 200.0, 200.0, 0.2, Rc::clone(&tex_black)),
                (150.0, 150.0, 200.0, 200.0, 0.5, Rc::clone(&tex_white)),
                (200.0, 200.0, 200.0, 200.0, 0.8, Rc::clone(&tex_black)),
            ];
            for (x, y, w, h, z, tex) in entities {
                let e = world.spawn();
                world.add_transform(e, mothra::ecs::Transform { x, y, w, h, z });
                world.add_sprite(e, mothra::ecs::Sprite { texture: tex });
            }
            println!("Created entities with varying z values.");
        }
    }

    fn render(&mut self, world: &World, renderer: &mut Renderer, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        renderer.draw_sprites_batched(encoder, view, world);
    }
}

fn main() {
    let mut logger_config = mothra::logger::LoggerConfig::default();
    logger_config.file_output = Some("log.txt".to_string());
    mothra::init_logger_with_config(logger_config);

    let config = GameConfig {
        window_width: 800,
        window_height: 600,
        logical_width: 800,
        logical_height: 600,
        title: "Test Sprite Z Order with Logger".to_string(),
        target_fps: 60,
        stretch_mode: false,
    };

    run_game(TestSpriteZ::new(), config);
}
