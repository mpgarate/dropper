use color::Color;
use game::Piece;

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
