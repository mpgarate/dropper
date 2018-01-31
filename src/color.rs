use rand::{thread_rng, Rng};

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
        let mut rng = thread_rng();

        rng.choose(&[
            Color::Red,
            Color::Yellow,
            Color::Blue,
            Color::Green,
            Color::Black,
        ]).unwrap().clone()
    }

    pub fn as_rgba(&self) -> [f32; 4] {
        match *self {
            Color::Red => [0.8, 0.0, 0.0, 1.0],
            Color::Yellow => [1.0, 1.0, 0.5, 1.0],
            Color::Blue => [0.0, 0.5, 1.0, 1.0],
            Color::Green => [0.0, 0.5, 0.0, 1.0],
            Color::Black => [0.0, 0.0, 0.0, 1.0],
        }
    }
}

