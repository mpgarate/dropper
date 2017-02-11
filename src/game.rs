use rand::{thread_rng, Rng};
use std::cmp;

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

        println!("in is_current_piece_blocked");
        self.pieces.get(row).and_then(|columns| columns.get(col)).unwrap_or(&None).is_some()
    }

    fn matches_color(&self, row: isize, col: isize) -> bool {
        let piece: Option<Piece> = self.pieces.get(row as usize).and_then(|columns| {
            println!("in matches_color");
            columns.get(col as usize).unwrap_or(&None).clone()
        });

        match piece {
            Some(ref p) => {
                self.current_piece.color == p.color
            }
            _ => false,
        }
    }

    fn remove_cleared_pieces(&mut self) {
        let row = self.current_piece.row as isize;
        let col = self.current_piece.col as isize;

        let clear_cases = [
            // vertical
            vec![
                (row, col),
                (row + 1, col),
                (row + 2, col),
                (row + 3, col),
            ],
            // horizontal
            vec![
                (row, col - 3),
                (row, col - 2),
                (row, col - 1),
                (row, col),
                (row, col + 1),
                (row, col + 2),
                (row, col + 3),
            ],
            // upward diagonal
            vec![
                (row - 3, col - 3),
                (row - 2, col - 2),
                (row - 1, col - 1),
                (row, col),
                (row + 1, col + 1),
                (row + 2, col + 2),
                (row + 3, col + 3),
            ],
            // downward diagonal
            vec![
                (row + 3, col - 3),
                (row + 2, col - 2),
                (row + 1, col - 1),
                (row, col),
                (row - 1, col + 1),
                (row - 2, col + 2),
                (row - 3, col + 3),
            ],
        ];


        for case in clear_cases.iter() {
            let sequence: Vec<_> = case.iter()
                .skip_while(|&&(row, col)| !self.matches_color(row, col))
                .take_while(|&&(row, col)| self.matches_color(row, col))
                .collect();

            if sequence.len() >= 4 {
                println!("got a seq");
                for &(row, col) in sequence {
                    println!("GOT HERE 2");
                    let mut row = self.pieces.get_mut(row as usize).unwrap();
                    row.push(None);
                    row.swap_remove(col as usize);
                }

                self.refresh_piece_rows_and_cols();
            } else {
                println!("too short {:?}", sequence.len());
            }
        }
    }

    fn refresh_piece_rows_and_cols(&mut self) {
        self.pieces = self.pieces.iter()
            .filter(|col| {
                col.iter().all(|piece| piece.is_some())
            })
            .enumerate()
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
                println!("step");
                panic!()
            }

            self.remove_cleared_pieces();
            self.current_piece = Piece::new();
        } else {
            self.current_piece.row += 1;
        }
    }

    pub fn move_left(&mut self) {
        // TODO: make sure we don't run into existing blocks
        if self.current_piece.col > 0 {
            self.current_piece.col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        // TODO: make sure we don't run into existing blocks
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
