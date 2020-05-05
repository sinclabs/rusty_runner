use ggez::*;
use ggez::event::{self, KeyCode};
use ggez::input;
use std::path;
use std::env;

struct GameState {
    ground_sprite_batch: graphics::spritebatch::SpriteBatch,
    runner_sprite_batch: graphics::spritebatch::SpriteBatch,
    runner_pose_rects: Vec<graphics::Rect>,
    runner_current_pose: usize,
    runner_position: f32,
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.runner_position += 1.0;
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.runner_position -= 0.5;
        }
        while timer::check_update_time(ctx, 10) {
            self.runner_current_pose += 1;
            if self.runner_current_pose > 9 {
                self.runner_current_pose = 0;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // Draw ground
        self.ground_sprite_batch.clear();
        for i in 0..30 {
            if i == 5 || i == 6 { continue; } 
            let p = graphics::DrawParam::new()
                .dest(nalgebra::Point2::new(75.0 * i as f32, 0.0))
                .scale(nalgebra::Vector2::new(0.5, 0.5));
            self.ground_sprite_batch.add(p);
        }
        let ground_draw_params = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(0.0, 525.0))
            .scale(nalgebra::Vector2::new(1.0, 1.0))
            .offset(nalgebra::Point2::new(750.0, 750.0));
        graphics::draw(ctx, &self.ground_sprite_batch, ground_draw_params)?;
        
        // Draw runner
        self.runner_sprite_batch.clear();
        let runner_clip_draw_params = graphics::DrawParam::new()
            .src(self.runner_pose_rects[self.runner_current_pose])
            .dest(nalgebra::Point2::new(0.0, 0.0))
            .scale(nalgebra::Vector2::new(0.22, 0.2));
        self.runner_sprite_batch.add(runner_clip_draw_params);
        let runner_draw_params = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(self.runner_position, 437.0));
        graphics::draw(ctx, &self.runner_sprite_batch, runner_draw_params)?;

        graphics::present(ctx)?;
        self.ground_sprite_batch.clear();
        Ok(())
    }
}

fn main() -> GameResult<()> {
    let resource_dir_path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let config = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Rusty Runner", "Subbu")
                                                            .conf(config)
                                                            .add_resource_path(resource_dir_path)
                                                            .build()
                                                            .unwrap();

    let ground_tile_image = graphics::Image::new(ctx, "/ground.png").unwrap();
    let runner_tile_image = graphics::Image::new(ctx, "/runner.png").unwrap();
    let state = &mut GameState {
        ground_sprite_batch: graphics::spritebatch::SpriteBatch::new(ground_tile_image),
        runner_sprite_batch: graphics::spritebatch::SpriteBatch::new(runner_tile_image),
        runner_pose_rects: vec![
            graphics::Rect::new(0.0, 0.0, 0.2, 0.5),
            graphics::Rect::new(0.2, 0.0, 0.2, 0.5),
            graphics::Rect::new(0.4, 0.0, 0.2, 0.5),
            graphics::Rect::new(0.6, 0.0, 0.2, 0.5),
            graphics::Rect::new(0.8, 0.0, 0.2, 0.5),
            graphics::Rect::new(0.0, 0.55, 0.2, 0.5),
            graphics::Rect::new(0.2, 0.55, 0.2, 0.5),
            graphics::Rect::new(0.4, 0.55, 0.2, 0.5),
            graphics::Rect::new(0.6, 0.55, 0.2, 0.5),
            graphics::Rect::new(0.8, 0.55, 0.2, 0.5),
        ],
        runner_current_pose: 0,
        runner_position: 0.0,
    };
    event::run(ctx, event_loop, state)
}
