extern crate dropper;
extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use dropper::frame_timer::FrameTimer;
use dropper::game::Game;

use std::time::{Duration};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "dropper",
            [600, 600]
        )
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let mut frame_timer = FrameTimer::new(Duration::from_millis(150));
    let mut game = Game::new();

    while let Some(e) = window.next() {
        if frame_timer.next_frame() {
            game.step();
        }

        window.draw_2d(&e, |c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            g.clear_stencil(0);

            Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                [game.current_piece_x(), game.current_piece_y(), 100.0, 100.0],
                &c.draw_state,
                c.transform,
                g
            );
        });

        if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
            game.move_left();
        }

        if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
            game.move_right();
        }
    }
}
