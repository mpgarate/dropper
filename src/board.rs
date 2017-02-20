use color::Color;
use game::Piece;


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
