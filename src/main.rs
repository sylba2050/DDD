extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use std::env;
use std::path;

struct MainState {
    tile: graphics::Image,
    message_frame_img: graphics::Image,
    frames: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let tile = graphics::Image::new(ctx, "/world.png")?;
        let message_frame_img = graphics::Image::new(ctx, "/hakkou1.png")?;
        let frames = 0;

        let s = MainState {
            tile,
            message_frame_img,
            frames,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::BLACK);
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::WHITE)?;
        graphics::draw_ex(
            ctx,
            &self.tile,
            graphics::DrawParam {
                src: graphics::Rect::new(3. / 8., 4. / 23., 1. / 8., 1. / 23.),
                dest: graphics::Point2::new(0., 0.),
                scale: graphics::Point2::new(2., 2.),
                .. Default::default()
            },
        ).expect("cannot draw tile");

        graphics::draw_ex(
            ctx,
            &self.message_frame_img,
            graphics::DrawParam {
                dest: graphics::Point2::new(300., 750.),
                scale: graphics::Point2::new(1.2, 1.),
                .. Default::default()
            },
        ).expect("cannot draw message_frame_img");

        graphics::set_color(ctx, graphics::Color::new(0., 0., 1., 1.))?;
        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect::new(0., 0., 280., 960.),
        )?;
    
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let ctx = &mut ContextBuilder::new("logging", "ggez")
        .window_setup(
            WindowSetup {
                title: "DDD".to_string(),
                .. Default::default()
            },
        )
        .window_mode(
            WindowMode {
                width: 1280,
                height: 960,
                .. Default::default()
            },
        )
        .build()
        .unwrap();
    
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
