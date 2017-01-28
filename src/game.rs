struct Point {
    x: u32,
    y: u32,
}

const INIT_POINT: Point = Point {x: 0, y: 0};

pub struct Game {
    current_piece: Point,
    pieces: Vec<Vec<Point>>,
    height: u32,
    width: u32,
}

impl Game {
    pub fn new(height: u32, width: u32) -> Game {
        Game {
            current_piece: INIT_POINT,
            pieces: vec![],
            height: height,
            width: width,
        }
    }

    pub fn step(&mut self) {
        self.current_piece.y += 1;

        if self.current_piece.y > self.height {
            self.current_piece.y = 0
        }
    }

    pub fn move_left(&mut self) {
        if self.current_piece.x > 0 {
            self.current_piece.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.current_piece.x < self.width - 1 {
            self.current_piece.x += 1;
        }
    }

    pub fn current_piece_x(&self) -> u32 {
        self.current_piece.x
    }

    pub fn current_piece_y(&self) -> u32 {
        self.current_piece.y
    }
}
