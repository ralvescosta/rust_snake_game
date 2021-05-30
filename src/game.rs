use piston_window::{types::Color, Context, G2d, Key};

use crate::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    food_exist: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            food_exist: true,
            food_x: 6,
            food_y: 4,

            width,
            height,

            game_over: false,
            waiting_time: 0.0,
        }
    }
}

impl Game {
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        // update snake
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        // draw snake

        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, ctx, g)
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, ctx, g); // border top
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g); // border bottom
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, ctx, g); // border left
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g); // border right

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g);
        }
    }
}
