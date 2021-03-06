extern crate ggez;

use ggez::event::{self, MouseButton, MouseState};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use std::{env, path, cmp};

fn limit(target: i32, under_limit: i32, upper_limit: i32) -> usize {
    cmp::max(cmp::min(target, upper_limit), under_limit) as usize
}

fn get_tile_img(num: u8, x: f32, y: f32) -> graphics::DrawParam {
    let mut p;
    match num {
        0 => {
                p = graphics::DrawParam {
                    src: graphics::Rect::new(3. / 8., 4. / 23., 1. / 8., 1. / 23.),
                    dest: graphics::Point2::new(x * 32., y * 32.),
                    scale: graphics::Point2::new(2., 2.),
                    .. Default::default()
                };
            },
        1 => {
                p = graphics::DrawParam {
                    src: graphics::Rect::new(7. / 8., 6. / 23., 1. / 8., 1. / 23.),
                    dest: graphics::Point2::new(x * 32., y * 32.),
                    scale: graphics::Point2::new(2., 2.),
                    .. Default::default()
                };
            },
        2 => {
                p = graphics::DrawParam {
                    src: graphics::Rect::new(0. / 8., 15. / 23., 1. / 8., 1. / 23.),
                    dest: graphics::Point2::new(x * 32., y * 32.),
                    scale: graphics::Point2::new(2., 2.),
                    .. Default::default()
                };
            },
        _ => {
                p = graphics::DrawParam {
                    src: graphics::Rect::new(3. / 8., 4. / 23., 1. / 8., 1. / 23.),
                    dest: graphics::Point2::new(x * 32., y * 32.),
                    scale: graphics::Point2::new(2., 2.),
                    .. Default::default()
                };
            },
    }
    p
}

struct FieldMoved {
    x: i32,
    y: i32,
}

struct MainState {
    field: [[u8; 100]; 100],
    tiles: graphics::spritebatch::SpriteBatch,
    bases: graphics::spritebatch::SpriteBatch,
    message_frame_img: graphics::Image,
    frames: usize,
    field_moved: FieldMoved,
    mouse_down: bool,
    FIELD_AREA_WIDTH: f32,
    FIELD_AREA_HEIGHT: f32,
    FIELD_AREA_X: f32,
    FIELD_AREA_Y: f32,
    NUM_FIELD_MESH_X: i32,
    NUM_FIELD_MESH_Y: i32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut field = [[0; 100]; 100];
        let tile = graphics::Image::new(ctx, "/world.png")?;
        let tiles = graphics::spritebatch::SpriteBatch::new(tile);
        let base = graphics::Image::new(ctx, "/world.png")?;
        let bases = graphics::spritebatch::SpriteBatch::new(base);
        let message_frame_img = graphics::Image::new(ctx, "/hakkou1.png")?;
        let frames = 0;
        let field_moved = FieldMoved {x: 0, y: 0};
        let mouse_down = false;

        const FIELD_AREA_WIDTH: f32 = 960.;
        const FIELD_AREA_HEIGHT: f32 = 736.;
        const FIELD_AREA_X: f32 = 300.;
        const FIELD_AREA_Y: f32 = 10.;
        const NUM_FIELD_MESH_X: i32 = 30;
        const NUM_FIELD_MESH_Y: i32 = 23;

        field[0][0] = 1;
        field[10][0] = 1;
        field[20][0] = 1;
        field[30][0] = 1;
        field[40][0] = 1;
        field[10][10] = 1;
        field[20][20] = 1;
        field[30][30] = 1;
        field[40][40] = 1;
        field[5][5] = 2;
        field[15][15] = 2;
        field[25][25] = 2;
        field[35][35] = 2;
        field[45][45] = 2;

        let s = MainState {
            field,
            tiles,
            bases,
            message_frame_img,
            frames,
            field_moved,
            mouse_down,
            FIELD_AREA_WIDTH,
            FIELD_AREA_HEIGHT,
            FIELD_AREA_X,
            FIELD_AREA_Y,
            NUM_FIELD_MESH_X,
            NUM_FIELD_MESH_Y,
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

        for x in 0..self.NUM_FIELD_MESH_X as usize {
            for y in 0..self.NUM_FIELD_MESH_Y as usize {
                let p = get_tile_img(0, x as f32, y as f32);
                self.bases.add(p);
            }
        }

        graphics::draw_ex(
            ctx,
            &self.bases,
            graphics::DrawParam {
                dest: graphics::Point2::new(self.FIELD_AREA_X, self.FIELD_AREA_Y),
                .. Default::default()
            },
        ).expect("cannot draw base");

        let field_start_x = limit(-(self.field_moved.x as f32 / 32.) as i32, 0, 100 - self.NUM_FIELD_MESH_X);
        let field_start_y = limit(-(self.field_moved.y as f32 / 32.) as i32, 0, 100 - self.NUM_FIELD_MESH_Y);
        self.field_moved.x = limit(self.field_moved.x, -(32 * (100 - self.NUM_FIELD_MESH_X)), 0)  as i32;
        self.field_moved.y = limit(self.field_moved.y, -(32 * (100 - self.NUM_FIELD_MESH_Y)), 0)  as i32;

        for (xf, x) in (field_start_x..(field_start_x + self.NUM_FIELD_MESH_X as usize)).enumerate() {
            for (yf, y) in (field_start_y..(field_start_y + self.NUM_FIELD_MESH_Y as usize)).enumerate() {
                if self.field[x][y] == 0 {
                    continue
                }
                let p = get_tile_img(self.field[x][y], xf as f32, yf as f32);
                self.tiles.add(p);
            }
        }

        graphics::draw_ex(
            ctx,
            &self.tiles,
            graphics::DrawParam {
                dest: graphics::Point2::new(self.FIELD_AREA_X, self.FIELD_AREA_Y),
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
            graphics::Rect::new(self.FIELD_AREA_X, self.FIELD_AREA_Y, self.FIELD_AREA_WIDTH, self.FIELD_AREA_HEIGHT),
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

        self.bases.clear();
        self.tiles.clear();
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
