use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Black,
}

impl Color {
    pub fn rand() -> Color {
        let items = [
            Color::Red,
            Color::Yellow,
            Color::Blue,
            Color::Green,
            Color::Black,
        ];

        items.choose(&mut thread_rng()).unwrap().clone()
    }

    pub fn as_rgba(&self) -> [f32; 4] {
        match *self {
            Color::Red => [0.6, 0.2, 0.2, 1.0],
            Color::Yellow => [0.7, 0.7, 0.25, 1.0],
            Color::Blue => [0.2, 0.4, 0.6, 1.0],
            Color::Green => [0.2, 0.6, 0.2, 1.0],
            Color::Black => [0.1, 0.1, 0.1, 1.0],
        }
    }
}
