struct Point {
    x: f64,
    y: f64,
}

pub struct Game {
    current_piece: Point
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_piece: Point {x: 0.0, y: 0.0},
        }
    }

    pub fn step(&mut self) {
        self.current_piece.y += 15.0;

        if self.current_piece.y > 500.0 {
            self.current_piece.y = 0.0
        }
    }

    pub fn move_left(&mut self) {
        self.current_piece.x -= 30.0;

        if self.current_piece.x < 0.0 {
            self.current_piece.x = 0.0
        }
    }

    pub fn move_right(&mut self) {
        self.current_piece.x += 30.0;

        if self.current_piece.x > 500.0 {
            self.current_piece.x = 500.0
        }
    }

    pub fn current_piece_x(&self) -> f64 {
        self.current_piece.x
    }

    pub fn current_piece_y(&self) -> f64 {
        self.current_piece.y
    }
}
