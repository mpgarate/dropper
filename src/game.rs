#[derive(Clone, Debug)]
pub enum Color {
    RED,
    YELLOW,
    BLUE,
    GREEN,
}

#[derive(Clone, Debug)]
pub struct Piece {
    col: usize,
    row: usize,
    color: Color,
}

impl Piece {
    pub fn new() -> Piece {
        // TODO: generate this randomly
        let color = Color::RED; 

         Piece {col: 0, row: 0, color: color}
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
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

    pub fn step(&mut self) {
        if self.is_current_piece_blocked() {
            if let Some(col) = self.pieces.get_mut(self.current_piece.row) {
                col[self.current_piece.col] = Some(self.current_piece.clone());
            } else {
                panic!()
            }

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
