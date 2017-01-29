use rand::{thread_rng, Rng};

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Yellow,
    Blue,
    Green,
}

impl Color {
    fn rand() -> Color {
        let mut rng = thread_rng();

        rng.choose(&[
            Color::Red,
            Color::Yellow,
            Color::Blue,
            Color::Green,
        ]).unwrap().clone()
    }

    pub fn as_rgba(&self) -> [f32; 4] {
        match *self {
            Color::Red => [0.8, 0.0, 0.0, 1.0],
            Color::Yellow => [1.0, 1.0, 0.5, 1.0],
            Color::Blue => [0.0, 0.5, 1.0, 1.0],
            Color::Green => [0.0, 0.5, 0.0, 1.0],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    col: usize,
    row: usize,
    color: Color,
}

impl Piece {
    pub fn new() -> Piece {
        let color = Color::rand();

         Piece {col: 0, row: 0, color: color}
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn color_rgba(&self) -> [f32; 4]{
        self.color.as_rgba()
    }
}

pub struct Game {
    current_piece: Piece,
    height: usize,
    width: usize,
    pieces: Vec<Vec<Option<Piece>>>,
}

impl Game {
    pub fn new(height: usize, width: usize) -> Game {
        Game {
            current_piece: Piece::new(),
            height: height,
            width: width,
            pieces: vec![vec![None; width]; height],
        }
    }

    fn is_current_piece_blocked(&self) -> bool {
        let col = self.current_piece.col;
        let row = self.current_piece.row + 1;

        if row >= self.height {
            return true
        }

        self.pieces.get(row).unwrap().get(col).unwrap().is_some()
    }

    fn remove_cleared_rows(&mut self) {
        let mut in_sequence = 0;

        for col in 0..4 {
            let piece = self.pieces.get(self.current_piece.row).unwrap().get(col).unwrap();

            if let Some(ref p) = *piece {
                if p.color == self.current_piece.color {
                    in_sequence += 1
                } else {
                    in_sequence = 0
                }
            }
        }

        if in_sequence < 4 {
            return
        }

        self.pieces.remove(self.current_piece.row);
        self.pieces.insert(0, vec![None; self.width]);

        self.refresh_piece_rows_and_cols();
    }

    fn refresh_piece_rows_and_cols(&mut self) {
        self.pieces = self.pieces.iter().enumerate()
            .map(|(row_index, col)| {
                col.iter().enumerate().map(|(col_index, piece)| {
                    if let Some(ref p) = *piece {
                        Some(Piece {
                            row: row_index,
                            col: col_index,
                            color: p.color.clone(),
                        })
                    } else {
                        None
                    }
                }).collect()
            }).collect();
    }

    pub fn step(&mut self) {
        if self.is_current_piece_blocked() {
            if let Some(col) = self.pieces.get_mut(self.current_piece.row) {
                col[self.current_piece.col] = Some(self.current_piece.clone());
            } else {
                panic!()
            }

            self.remove_cleared_rows();
            self.current_piece = Piece::new();
        } else {
            self.current_piece.row += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.current_piece.col > 0 {
            self.current_piece.col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.current_piece.col < self.width - 1 {
            self.current_piece.col += 1;
        }
    }

    pub fn get_pieces(&self) -> Vec<Piece> {
        self.pieces.iter()
            .flat_map(|col| col.iter().flat_map(|piece| piece.clone()))
            .chain(vec![self.current_piece.clone()])
            .collect()
    }

    pub fn current_piece_col(&self) -> usize {
        self.current_piece.col
    }

    pub fn current_piece_row(&self) -> usize {
        self.current_piece.row
    }
}
