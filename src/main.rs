extern crate piston_window;
extern crate rand;

use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (500, 500);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            println!("{:?}", key)
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
        });

        event.update(|arg| {});
    }
}

fn to_coord_u32(c: i32) -> u32 {
    return c as u32;
}
