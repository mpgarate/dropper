use clear_strategy::ClearStrategy::*;
use std::iter;

pub enum ClearStrategy {
    Vertical,
    Horizontal,
    DiagonalUp,
    DiagonalDown,
}

impl ClearStrategy {
    pub fn get_starting_points(&self, height: usize, width: usize) -> Vec<(usize, usize)> {
        match *self {
            Vertical => (iter::repeat(0)).zip(0..width).collect(),
            Horizontal => (0..height).zip(iter::repeat(0)).collect(),
            DiagonalUp => ((0..height).zip(iter::repeat(0)))
                .chain(iter::repeat(height - 1).zip(1..width))
                .collect(),
            DiagonalDown => (iter::repeat(0))
                .zip(0..width)
                .chain((1..height).zip(iter::repeat(0)))
                .collect(),
        }
    }

    pub fn get_next_point(
        &self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
    ) -> Option<(usize, usize)> {
        let (new_row, new_col) = match *self {
            Vertical => (row + 1, col),
            Horizontal => (row, col + 1),
            DiagonalUp if row > 0 => (row - 1, col + 1),
            DiagonalDown => (row + 1, col + 1),
            _ => return None,
        };

        let is_invalid_point = new_row >= height || new_col >= width;

        if is_invalid_point {
            None
        } else {
            Some((new_row, new_col))
        }
    }
}
