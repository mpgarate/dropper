use clear_strategy::ClearStrategy::*;
use std::iter;

pub enum ClearStrategy {
    Vertical,
    Horizontal,
}

impl ClearStrategy {
    pub fn get_starting_points(&self, height: usize, width: usize) -> Vec<(usize, usize)> {
        match *self {
            Vertical => (iter::repeat(0)).zip(0..width).collect(),
            Horizontal => (0..height).zip(iter::repeat(0)).collect(),
        }
    }

    pub fn get_next_point(&self, row: usize, col: usize) -> (usize, usize) {
        match *self {
            Vertical => (row + 1, col),
            Horizontal => (row, col + 1),
        }
    }
}
