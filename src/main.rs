extern crate ggez;

use ggez::event::{self, MouseButton, MouseState};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use std::env;
use std::path;

struct FieldMoved {
    x: i32,
    y: i32,
}

struct MainState {
    field: [[u8; 100]; 100],
    tiles: graphics::spritebatch::SpriteBatch,
    message_frame_img: graphics::Image,
    frames: usize,
    field_moved: FieldMoved,
    mouse_down: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let field = [[0; 100]; 100];
        let tile = graphics::Image::new(ctx, "/world.png")?;
        let tiles = graphics::spritebatch::SpriteBatch::new(tile);
        let message_frame_img = graphics::Image::new(ctx, "/hakkou1.png")?;
        let frames = 0;
        let field_moved = FieldMoved {x: 0, y: 0};
        let mouse_down = false;

        let s = MainState {
            field,
            tiles,
            message_frame_img,
            frames,
            field_moved,
            mouse_down,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.mouse_down = true;
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.mouse_down = false;
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState, _x: i32, _y: i32, xrel: i32, yrel: i32,) {
        if self.mouse_down {
            self.field_moved.x += xrel;
            self.field_moved.y += yrel;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::BLACK);
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::WHITE)?;
        let mut p;
        for x in 0..self.field.len() {
            for y in 0..self.field[x].len() {
                let xf = x as f32;
                let yf = y as f32;
                match self.field[x][y] {
                    0 => p = graphics::DrawParam {
                            src: graphics::Rect::new(3. / 8., 4. / 23., 1. / 8., 1. / 23.),
                            dest: graphics::Point2::new(xf * 32., yf * 32.),
                            scale: graphics::Point2::new(2., 2.),
                            .. Default::default()
                        },
                    _ => {
                            p = graphics::DrawParam {
                                src: graphics::Rect::new(7. / 8., 0. / 23., 1. / 8., 1. / 23.),
                                dest: graphics::Point2::new(xf * 32., yf * 32.),
                                scale: graphics::Point2::new(2., 2.),
                                .. Default::default()
                            };
                            println!("undefined number");
                        },
                }
                self.tiles.add(p);
            }
        }
        graphics::draw_ex(
            ctx,
            &self.tiles,
            graphics::DrawParam {
                dest: graphics::Point2::new(300. + self.field_moved.x as f32, 10. + self.field_moved.y as f32),
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

        graphics::rectangle(
            ctx,
            graphics::DrawMode::Line(1.),
            graphics::Rect::new(300., 10., 970., 740.),
        )?;

        graphics::set_color(ctx, graphics::Color::new(0., 0., 1., 1.))?;
        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect::new(0., 0., 280., 740.),
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
