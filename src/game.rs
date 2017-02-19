use color::Color;
use rand::{thread_rng, Rng};

pub enum MoveDirection {
    Left,
    Right
}

pub enum PieceGenerator{
    Random(usize),
    Exact(Vec<Piece>),
}

// TODO refactor this into a trait
impl PieceGenerator {
    pub fn next(&mut self) -> Piece {
        match self {
            &mut PieceGenerator::Random(max_col) => {
                let mut rng = thread_rng();

                Piece {
                    row: 0,
                    col: rng.gen_range(0, max_col),
                    color: Color::rand(),
                }
            },
            &mut PieceGenerator::Exact(ref mut pieces) => pieces.remove(0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    row: usize,
    col: usize,
    color: Color,
}

impl Piece {
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
    piece_generator: PieceGenerator,
    board: Vec<Vec<Option<Color>>>,
    num_rows_cleared: u64,
    width: usize,
}

impl Game {
    pub fn new(height: usize, width: usize, mut piece_generator: PieceGenerator) -> Game {
        let piece = piece_generator.next();

        Game {
            piece_generator: piece_generator,
            current_piece: piece,
            board: vec![vec![None; width]; height],
            num_rows_cleared: 0,
            width: width,
        }
    }

    pub fn drop_piece(&mut self) {
        let col = self.current_piece.col();
        let color = self.current_piece.color.clone();

        let first_free_row = self.board
            .iter()
            .rposition(|row| row.get(col.clone()).unwrap_or(&None).is_none())
            .unwrap();

        self.board.get_mut(first_free_row).unwrap()[col] = Some(color);

        self.current_piece = self.piece_generator.next();
    }

    pub fn get_pieces(&self) -> Vec<Piece> {
        self.board.iter()
            .enumerate()
            .flat_map(|(row_num, col)| {
                let pieces: Vec<Piece> = col.iter()
                    .enumerate()
                    .filter_map(|(col_num, color)| {
                        if let Some(c) = color.clone() {
                            Some(Piece { row: row_num, col: col_num, color: c })
                        } else {
                            None
                        }
                    }).collect();
                pieces
            })
            .chain(vec![self.current_piece.clone()])
            .collect()
    }

    pub fn move_piece(&mut self, direction: MoveDirection) {
        let col = self.current_piece.col();

        let new_col = match direction {
            MoveDirection::Left if col > 0 => col - 1,
            MoveDirection::Right if col < self.width - 1 => col + 1,
            _ => col,
        };

        self.current_piece.col = new_col;
    }

    pub fn step(&mut self) {
    }

    pub fn num_rows_cleared(&self) -> u64 {
        self.num_rows_cleared
    }
}

#[cfg(test)]
mod tests {
    use game::Game;
    use game::MoveDirection;
    use game::Piece;
    use game::PieceGenerator;
    use color::Color;

    const HEIGHT: usize = 16;
    const WIDTH: usize = 4;

    #[test]
    fn move_right() {
        let pieces_to_drop = vec![
            Piece { row: 0, col: 0, color: Color::Red },
        ];

        let expected_pieces = vec![
            Piece { row: 0, col: 1, color: Color::Red },
        ];

        let mut game = Game::new(
            HEIGHT,
            WIDTH,
            PieceGenerator::Exact(pieces_to_drop),
        );

        game.move_piece(MoveDirection::Right);

        assert_eq!(expected_pieces, game.get_pieces());
    }

    #[test]
    fn move_left() {
        let pieces_to_drop = vec![
            Piece { row: 0, col: 3, color: Color::Red },
        ];

        let expected_pieces = vec![
            Piece { row: 0, col: 2, color: Color::Red },
        ];

        let mut game = Game::new(
            HEIGHT,
            WIDTH,
            PieceGenerator::Exact(pieces_to_drop),
        );

        game.move_piece(MoveDirection::Left);

        assert_eq!(expected_pieces, game.get_pieces());
    }

    #[test]
    fn move_right_stops_at_game_edge() {
        let pieces_to_drop = vec![
            Piece { row: 0, col: WIDTH - 1, color: Color::Red },
        ];

        let expected_pieces = vec![
            Piece { row: 0, col: WIDTH - 1, color: Color::Red },
        ];

        let mut game = Game::new(
            HEIGHT,
            WIDTH,
            PieceGenerator::Exact(pieces_to_drop),
        );

        game.move_piece(MoveDirection::Right);

        assert_eq!(expected_pieces, game.get_pieces());
    }

    #[test]
    fn move_left_stops_at_game_edge() {
        let pieces_to_drop = vec![
            Piece { row: 0, col: 0, color: Color::Red },
        ];

        let expected_pieces = vec![
            Piece { row: 0, col: 0, color: Color::Red },
        ];

        let mut game = Game::new(
            HEIGHT,
            WIDTH,
            PieceGenerator::Exact(pieces_to_drop),
        );

        game.move_piece(MoveDirection::Left);

        assert_eq!(expected_pieces, game.get_pieces());
    }

    #[test]
    fn drop_piece_empty_board() {
        let pieces_to_drop = vec![
            Piece { row: 0, col: 0, color: Color::Red },
            Piece { row: 0, col: 0, color: Color::Yellow },
        ];

        let expected_pieces = vec![
            Piece { row: HEIGHT - 1, col: 0, color: Color::Red },
            Piece { row: 0, col: 0, color: Color::Yellow },
        ];

        let mut game = Game::new(
            HEIGHT,
            WIDTH,
            PieceGenerator::Exact(pieces_to_drop),
        );

        game.drop_piece();

        assert_eq!(expected_pieces, game.get_pieces());
    }

    #[test]
    fn drop_piece_on_another_piece() {
        let pieces_to_drop = vec![
            Piece { row: 0, col: 0, color: Color::Yellow },
            Piece { row: 0, col: 0, color: Color::Red },
            Piece { row: 0, col: 0, color: Color::Blue },
        ];

        let mut expected_pieces = vec![
            Piece { row: HEIGHT - 2, col: 0, color: Color::Red },
            Piece { row: HEIGHT - 1, col: 0, color: Color::Yellow },
            Piece { row: 0, col: 0, color: Color::Blue },
        ];

        let mut game = Game::new(
            HEIGHT,
            WIDTH,
            PieceGenerator::Exact(pieces_to_drop),
        );

        game.drop_piece();
        game.drop_piece();

        let mut actual_pieces = game.get_pieces();

        expected_pieces.sort();
        actual_pieces.sort();

        assert_eq!(expected_pieces, actual_pieces);
    }
}
