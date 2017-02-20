use color::Color;
use game::Piece;

use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    color_matrix: Vec<Vec<Option<Color>>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            color_matrix: vec![vec![None; width]; height],
        }
    }

    fn height(&self) -> usize {
        self.color_matrix.len()
    }

    fn width(&self) -> usize {
        self.color_matrix.get(0).unwrap_or(&vec![]).len()
    }

    pub fn set(&mut self, row: usize, col: usize, color: Color) {
        self.color_matrix.get_mut(row).unwrap()[col] = Some(color);
    }

    pub fn get(&self, row: usize, col:usize) -> Option<Color> {
        self.color_matrix.get(row)
            .unwrap_or(&vec![])
            .get(col)
            .unwrap_or(&None)
            .clone()
    }

    pub fn get_pieces(&self) -> Vec<Piece> {
        self.color_matrix.iter()
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
            .collect()
    }

    fn get_sequences_in_coordinate_list(
        &self,
        row_range: Range<usize>,
        col_range: Range<usize>,
    ) -> Vec<Vec<Piece>>
        {
        println!("callin it!");

        let sequence_clear_len = 4;

        let mut sequences: Vec<Vec<Piece>> = vec![];

        let mut sequence: Vec<Piece> = vec![];

        for row in row_range {
            for col in col_range.clone() {
                let color = self.get(row, col);

                let prev_color = match sequence.last() {
                    Some(piece) => Some(piece.color()),
                    _ => None,
                };

                match (color, prev_color) {
                    (Some(ref c1), Some(ref c2)) if c1 == c2 => {
                        sequence.push(Piece {row: row, col: col, color: c1.clone() });
                    },
                    (Some(c1), None) => {
                        if sequence.len() >= sequence_clear_len {
                            sequences.push(sequence.clone());
                        }

                        sequence.clear();
                        sequence.push(Piece {row: row, col: col, color: c1.clone() });
                    },
                    _ => {
                        if sequence.len() >= sequence_clear_len {
                            sequences.push(sequence.clone());
                        }

                        sequence.clear();
                    }
                }
            }
        }

        if sequence.len() >= sequence_clear_len {
            sequences.push(sequence);
        }

        sequences
    }

    pub fn get_sequences_to_clear(&self) -> Vec<Vec<Piece>> {
        let mut sequences: Vec<Vec<Piece>> = vec![];

        // vertical
        for col in 0..self.width() {
            let new_sequences = self.get_sequences_in_coordinate_list(
                (0..self.height()),
                (col..col+ 1),
            );

            for s in new_sequences {
                sequences.push(s);
            }
        }

        // horizontal
        for row in 0..self.height() {
            let new_sequences = self.get_sequences_in_coordinate_list(
                (row..row+ 1),
                (0..self.width()),
            );

            for s in new_sequences {
                sequences.push(s);
            }
        }

        sequences
    }

    pub fn clear_all(&mut self, pieces: Vec<Piece>) {
    }

    pub fn get_lowest_free_row_in_col(&self, col: usize) -> usize {
        self.color_matrix.iter()
            .rposition(|row| row.get(col).unwrap_or(&None).is_none())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use color::Color::*;
    use game::Piece;

    #[test]
    fn get_sequences_to_clear_horizontal() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None, None, None ],
                vec![ None, None, None, None ],
                vec![ Some(Red), Some(Red), Some(Red), Some(Red) ],
                vec![ None, None, None, None ],
            ],
        };

        let expected_sequences_to_clear = vec![vec![
            Piece { row: 2, col: 0, color: Red },
            Piece { row: 2, col: 1, color: Red },
            Piece { row: 2, col: 2, color: Red },
            Piece { row: 2, col: 3, color: Red },
        ]];

        let sequences_to_clear = board.get_sequences_to_clear();

        assert_eq!(expected_sequences_to_clear, sequences_to_clear);
    }

    #[test]
    fn get_sequences_to_clear_vertical() {
        let board = Board {
            color_matrix: vec![
                vec![ Some(Red), None ],
                vec![ Some(Red), None ],
                vec![ Some(Red), None ],
                vec![ Some(Red), None ],
                vec![ Some(Yellow), None ],
            ],
        };

        let expected_sequences_to_clear = vec![vec![
            Piece { row: 0, col: 0, color: Red },
            Piece { row: 1, col: 0, color: Red },
            Piece { row: 2, col: 0, color: Red },
            Piece { row: 3, col: 0, color: Red },
        ]];

        let sequences_to_clear = board.get_sequences_to_clear();

        assert_eq!(expected_sequences_to_clear, sequences_to_clear);
    }

    #[test]
    fn get_sequences_to_clear_vertical_none_eligible() {
        let board = Board {
            color_matrix: vec![
                vec![ Some(Red), None ],
                vec![ Some(Blue), None ],
                vec![ Some(Red), None ],
                vec![ Some(Red), None ],
            ],
        };

        let expected_sequences_to_clear: Vec<Vec<Piece>> = vec![];

        let sequences_to_clear = board.get_sequences_to_clear();

        assert_eq!(expected_sequences_to_clear, sequences_to_clear);
    }

    #[test]
    fn set_a_color() {
        let mut board = Board {
            color_matrix: vec![
                vec![ Some(Red), Some(Red) ],
                vec![ Some(Red), Some(Red) ],
            ],
        };

        board.set(1, 1, Yellow);

        let expected_board = Board {
            color_matrix: vec![
                vec![ Some(Red), Some(Red) ],
                vec![ Some(Red), Some(Yellow) ],
            ],
        };

        assert_eq!(expected_board, board);
    }

    #[test]
    fn get_a_color() {
        let board = Board {
            color_matrix: vec![
                vec![ Some(Red), Some(Blue) ],
                vec![ Some(Red), Some(Yellow) ],
            ],
        };

        let color = board.get(0, 1);

        let expected_color = Some(Blue);

        assert_eq!(expected_color, color);
    }

    #[test]
    fn get_lowest_free_row_in_col() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None ],
                vec![ None, None ],
            ],
        };

        let row = board.get_lowest_free_row_in_col(0);
        let expected_row = 1;

        assert_eq!(expected_row, row);
    }

    #[test]
    fn get_lowest_free_row_in_col_with_another_existing_piece() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None ],
                vec![ Some(Red), None ],
            ],
        };

        let row = board.get_lowest_free_row_in_col(0);
        let expected_row = 0;

        assert_eq!(expected_row, row);
    }
}
