extern crate dropper;
extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use dropper::frame_timer::FrameTimer;
use dropper::game::Game;

use std::time::{Duration};

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;

const BLOCK_WIDTH: u32 = 64;
const BLOCK_HEIGHT: u32 = 16;

const GAME_HEIGHT: u32 = WINDOW_HEIGHT / BLOCK_HEIGHT;
const GAME_WIDTH: u32 = WINDOW_WIDTH / BLOCK_WIDTH;

const FRAME_RATE: u64 = 50; 

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "dropper",
            [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32]
        )
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let mut frame_timer = FrameTimer::new(Duration::from_millis(FRAME_RATE));
    let mut game = Game::new(GAME_HEIGHT as usize, GAME_WIDTH as usize);

    while let Some(e) = window.next() {
        if frame_timer.next_frame() {
            game.step();
        }


        window.draw_2d(&e, |c, g| {
            for piece in game.get_pieces() {
                clear([0.8, 0.8, 0.8, 1.0], g);
                g.clear_stencil(0);

                Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                    [
                        (piece.col() as u32 * BLOCK_WIDTH) as f64,
                        (piece.row() as u32 * BLOCK_HEIGHT) as f64,
                        BLOCK_WIDTH as f64,
                        BLOCK_HEIGHT as f64,
                    ],
                    &c.draw_state,
                    c.transform,
                    g
                );
            }
        });

        if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
            game.move_left();
        }

        if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
            game.move_right();
        }
    }
}
