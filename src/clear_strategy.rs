use clear_strategy::ClearStrategy::*;
use std::iter;

pub enum ClearStrategy {
    Vertical,
    Horizontal,
    DiagonalUp,
}

impl ClearStrategy {
    pub fn get_starting_points(&self, height: usize, width: usize) -> Vec<(usize, usize)> {
        match *self {
            Vertical => (iter::repeat(0)).zip(0..width).collect(),
            Horizontal => (0..height).zip(iter::repeat(0)).collect(),
            DiagonalUp => ((0..height).zip(iter::repeat(0))).chain(
                    iter::repeat(height - 1).zip(1..width)
                ).collect(),
        }
    }

    pub fn get_next_point(
        &self,
        row: usize,
        col: usize,
        height: usize,
        width: usize
    ) -> Option<(usize, usize)> {
        let is_invalid_point = |row, col| row >= height || col >= width;

        let (new_row, new_col) = match *self {
            Vertical => (row + 1, col),
            Horizontal => (row, col + 1),
            DiagonalUp => {
                if row == 0 {
                    return None
                }

                (row - 1, col + 1)
            }
        };

        if is_invalid_point(new_row, new_col) {
            None
        } else {
            Some((new_row, new_col))
        }
    }
}
