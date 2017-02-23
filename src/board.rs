use color::Color;
use game::Piece;
use clear_strategy::ClearStrategy;

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

    fn unset(&mut self, row: usize, col: usize) {
        self.color_matrix.get_mut(row).unwrap()[col] = None;
    }

    fn remove_and_shift_column(&mut self, starting_row: usize, col: usize) {
        for row in (starting_row..self.height()).rev() {
            let piece_above = if row > 0 {
                self.get(row - 1, col)
            } else {
                None
            };

            self.color_matrix.get_mut(row).unwrap()[col] = piece_above;
        }
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

    fn get_sequential_pieces(&self, coordinates: Vec<(usize, usize)>) -> Vec<Piece> {
        let mut sequence_coordinates: Vec<&(usize, usize)> = coordinates.windows(4)
            .filter(|sequence| {
                sequence.get(0).map_or(false, |&(row_0, col_0)| {
                    self.get(row_0, col_0).map_or(false, |first_color| {
                        sequence.iter().all(|&(row, col)| {
                            self.get(row, col) == Some(first_color.clone())
                        })
                    })
                })
            })
            .flat_map(|s| s)
            .collect();

            sequence_coordinates.sort();
            sequence_coordinates.dedup();

            sequence_coordinates.iter().map(|&&(row, col)| {
                Piece { row: row, col: col, color: self.get(row, col).unwrap() }
            }).collect()
    }

    pub fn get_pieces_to_clear(&self) -> Vec<Piece> {
        let height = self.height();
        let width = self.width();

        let strategies = vec![
            ClearStrategy::Vertical,
            ClearStrategy::Horizontal,
            ClearStrategy::DiagonalUp,
            ClearStrategy::DiagonalDown,
        ];

        let mut pieces: Vec<Piece> = vec![];

        for strategy in strategies {
            let starting_points = strategy.get_starting_points(height, width);

            for (mut row, mut col) in starting_points {
                let mut coordinates = vec![];

                coordinates.push((row, col));

                while let Some(point) = strategy.get_next_point(
                    row, col, height, width
                ) {

                    row = point.0;
                    col = point.1;

                    coordinates.push((row, col));
                }

                let new_pieces = self.get_sequential_pieces(coordinates);
                for p in new_pieces {
                    pieces.push(p);
                }
            }
        }

        pieces
    }

    pub fn clear_all(&mut self, pieces: Vec<Piece>) {
        for piece in pieces {
            self.unset(piece.row(), piece.col());
        }

        for col in 0..self.width() {
            for row in (0..self.height()).rev() {
                let color = self.get(row, col);
                let color_above = self.get(row + 1, col);

                if let (None, Some(_)) = (color, color_above) {
                    self.remove_and_shift_column(row, col);
                }
            }
        }
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
    fn delete_all_horizontal_row() {
        let mut board = Board {
            color_matrix: vec![
                vec![ None, None, None, None ],
                vec![ None, None, None, None ],
                vec![ None, Some(Red), None, None ],
                vec![ Some(Red), Some(Red), Some(Red), Some(Red) ],
            ],
        };

        let pieces_to_clear = vec![
            Piece { row: 3, col: 0, color: Red },
            Piece { row: 3, col: 1, color: Red },
            Piece { row: 3, col: 2, color: Red },
            Piece { row: 3, col: 3, color: Red },
        ];

        let expected_board = Board {
            color_matrix: vec![
                vec![ None, None, None, None ],
                vec![ None, None, None, None ],
                vec![ None, None, None, None ],
                vec![ None, Some(Red), None, None ],
            ],
        };

        board.clear_all(pieces_to_clear);

        assert_eq!(expected_board, board);
    }

    #[test]
    fn get_pieces_to_clear_diagonal_down() {
        let board = Board {
            color_matrix: vec![
                vec![ Some(Red), None, None, None ],
                vec![ None, Some(Red), None, None ],
                vec![ None, None, Some(Red), None ],
                vec![ None, None, None, Some(Red) ],
            ],
        };

        let mut expected_pieces_to_clear = vec![
            Piece { row: 0, col: 0, color: Red },
            Piece { row: 1, col: 1, color: Red },
            Piece { row: 2, col: 2, color: Red },
            Piece { row: 3, col: 3, color: Red },
        ];

        let mut pieces_to_clear = board.get_pieces_to_clear();

        expected_pieces_to_clear.sort();
        pieces_to_clear.sort();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
    }

    #[test]
    fn get_pieces_to_clear_diagonal_down_off_center_down() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None, None, None, None],
                vec![ Some(Red), None, None, None ],
                vec![ None, Some(Red), None, None ],
                vec![ None, None, Some(Red), None ],
                vec![ None, None, None, Some(Red) ],
            ],
        };

        let mut expected_pieces_to_clear = vec![
            Piece { row: 1, col: 0, color: Red },
            Piece { row: 2, col: 1, color: Red },
            Piece { row: 3, col: 2, color: Red },
            Piece { row: 4, col: 3, color: Red },
        ];

        let mut pieces_to_clear = board.get_pieces_to_clear();

        expected_pieces_to_clear.sort();
        pieces_to_clear.sort();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
    }

    #[test]
    fn get_pieces_to_clear_diagonal_down_off_center_right() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None, None, None, None ],
                vec![ None, Some(Red), None, None, None ],
                vec![ None, None, Some(Red), None, None ],
                vec![ None, None, None, Some(Red), None ],
                vec![ None, None, None, None, Some(Red) ],
            ],
        };

        let mut expected_pieces_to_clear = vec![
            Piece { row: 1, col: 1, color: Red },
            Piece { row: 2, col: 2, color: Red },
            Piece { row: 3, col: 3, color: Red },
            Piece { row: 4, col: 4, color: Red },
        ];

        let mut pieces_to_clear = board.get_pieces_to_clear();

        expected_pieces_to_clear.sort();
        pieces_to_clear.sort();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
    }

    #[test]
    fn get_pieces_to_clear_diagonal_up() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None, None, Some(Red) ],
                vec![ None, None, Some(Red), None ],
                vec![ None, Some(Red), None, None ],
                vec![ Some(Red), None, None, None ],
            ],
        };

        let mut expected_pieces_to_clear = vec![
            Piece { row: 3, col: 0, color: Red },
            Piece { row: 2, col: 1, color: Red },
            Piece { row: 1, col: 2, color: Red },
            Piece { row: 0, col: 3, color: Red },
        ];

        let mut pieces_to_clear = board.get_pieces_to_clear();

        expected_pieces_to_clear.sort();
        pieces_to_clear.sort();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
    }

    #[test]
    fn get_pieces_to_clear_diagonal_up_off_center_up() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None, None, Some(Red), None ],
                vec![ None, None, Some(Red), None, None ],
                vec![ None, Some(Red), None, None, None ],
                vec![ Some(Red), None, None, None, None ],
                vec![ None, None, None, None, None ],
            ],
        };

        let mut expected_pieces_to_clear = vec![
            Piece { row: 3, col: 0, color: Red },
            Piece { row: 2, col: 1, color: Red },
            Piece { row: 1, col: 2, color: Red },
            Piece { row: 0, col: 3, color: Red },
        ];

        let mut pieces_to_clear = board.get_pieces_to_clear();

        expected_pieces_to_clear.sort();
        pieces_to_clear.sort();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
    }

    #[test]
    fn get_pieces_to_clear_diagonal_up_off_center_down() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None, None, None, None ],
                vec![ None, None, None, None, Some(Red) ],
                vec![ None, None, None, Some(Red), None ],
                vec![ None, None, Some(Red), None, None ],
                vec![ None, Some(Red), None, None, None ],
            ],
        };

        let mut expected_pieces_to_clear = vec![
            Piece { row: 4, col: 1, color: Red },
            Piece { row: 3, col: 2, color: Red },
            Piece { row: 2, col: 3, color: Red },
            Piece { row: 1, col: 4, color: Red },
        ];

        let mut pieces_to_clear = board.get_pieces_to_clear();

        expected_pieces_to_clear.sort();
        pieces_to_clear.sort();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
    }

    #[test]
    fn get_pieces_to_clear_horizontal() {
        let board = Board {
            color_matrix: vec![
                vec![ None, None, None, None ],
                vec![ None, None, None, None ],
                vec![ Some(Red), Some(Red), Some(Red), Some(Red) ],
                vec![ None, None, None, None ],
            ],
        };

        let expected_pieces_to_clear = vec![
            Piece { row: 2, col: 0, color: Red },
            Piece { row: 2, col: 1, color: Red },
            Piece { row: 2, col: 2, color: Red },
            Piece { row: 2, col: 3, color: Red },
        ];

        let pieces_to_clear = board.get_pieces_to_clear();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
    }

    #[test]
    fn get_pieces_to_clear_vertical() {
        let board = Board {
            color_matrix: vec![
                vec![ Some(Red), None ],
                vec![ Some(Red), None ],
                vec![ Some(Red), None ],
                vec![ Some(Red), None ],
                vec![ Some(Yellow), None ],
            ],
        };

        let expected_pieces_to_clear = vec![
            Piece { row: 0, col: 0, color: Red },
            Piece { row: 1, col: 0, color: Red },
            Piece { row: 2, col: 0, color: Red },
            Piece { row: 3, col: 0, color: Red },
        ];

        let pieces_to_clear = board.get_pieces_to_clear();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
    }

    #[test]
    fn get_pieces_to_clear_vertical_none_eligible() {
        let board = Board {
            color_matrix: vec![
                vec![ Some(Red), None ],
                vec![ Some(Blue), None ],
                vec![ Some(Red), None ],
                vec![ Some(Red), None ],
            ],
        };

        let expected_pieces_to_clear: Vec<Piece> = vec![];

        let pieces_to_clear = board.get_pieces_to_clear();

        assert_eq!(expected_pieces_to_clear, pieces_to_clear);
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
