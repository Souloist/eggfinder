//! Board state and operations.

use std::collections::HashSet;
use std::fmt;

use rand::Rng;

use crate::constants::CellType;

/// Coordinate type alias (row, col).
pub type Coordinate = (usize, usize);

/// Errors that can occur when creating or manipulating a board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardError {
    InvalidDimensions { width: usize, height: usize },
    TooManyEggs { egg_count: usize, max_cells: usize },
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::InvalidDimensions { width, height } => {
                write!(
                    f,
                    "Board dimensions must be positive, got {}x{}",
                    width, height
                )
            }
            BoardError::TooManyEggs {
                egg_count,
                max_cells,
            } => {
                write!(
                    f,
                    "Egg count {} exceeds board size {}",
                    egg_count, max_cells
                )
            }
        }
    }
}

impl std::error::Error for BoardError {}

/// Board state containing the grid, revealed cells, and egg locations.
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub egg_count: usize,
    /// Cell values: -1 = egg, 0 = empty, 1-8 = adjacent egg count.
    pub cells: Vec<Vec<i8>>,
    pub revealed: Vec<Vec<bool>>,
    pub eggs: HashSet<Coordinate>,
}

impl Board {
    /// Create a new board with random egg placement.
    ///
    /// # Errors
    /// Returns `BoardError::InvalidDimensions` if width or height is 0.
    /// Returns `BoardError::TooManyEggs` if egg_count exceeds board size.
    pub fn new(width: usize, height: usize, egg_count: usize) -> Result<Self, BoardError> {
        if width == 0 || height == 0 {
            return Err(BoardError::InvalidDimensions { width, height });
        }

        let max_cells = width * height;
        if egg_count > max_cells {
            return Err(BoardError::TooManyEggs {
                egg_count,
                max_cells,
            });
        }

        let mut board = Board {
            width,
            height,
            egg_count,
            cells: vec![vec![0; width]; height],
            revealed: vec![vec![false; width]; height],
            eggs: HashSet::new(),
        };

        board.place_eggs();
        board.calculate_numbers();
        Ok(board)
    }

    /// Place eggs randomly on the board.
    fn place_eggs(&mut self) {
        let mut rng = rand::rng();
        let mut placed = 0;

        while placed < self.egg_count {
            let row = rng.random_range(0..self.height);
            let col = rng.random_range(0..self.width);

            if !self.eggs.contains(&(row, col)) {
                self.eggs.insert((row, col));
                self.cells[row][col] = CellType::Egg.value();
                placed += 1;
            }
        }
    }

    /// Calculate adjacent egg counts for each non-egg cell.
    fn calculate_numbers(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.cells[row][col] != CellType::Egg.value() {
                    self.cells[row][col] = self.count_adjacent_eggs(row, col);
                }
            }
        }
    }

    fn count_adjacent_eggs(&self, row: usize, col: usize) -> i8 {
        self.get_neighbors(row, col)
            .iter()
            .filter(|&&(r, c)| self.is_egg(r, c))
            .count() as i8
    }

    pub fn is_valid_position(&self, row: usize, col: usize) -> bool {
        row < self.height && col < self.width
    }

    /// Get the cell value at a position. Returns None if out of bounds.
    pub fn get_cell(&self, row: usize, col: usize) -> Option<i8> {
        if self.is_valid_position(row, col) {
            Some(self.cells[row][col])
        } else {
            None
        }
    }

    pub fn is_egg(&self, row: usize, col: usize) -> bool {
        self.eggs.contains(&(row, col))
    }

    pub fn is_revealed(&self, row: usize, col: usize) -> bool {
        if self.is_valid_position(row, col) {
            self.revealed[row][col]
        } else {
            false
        }
    }

    /// Get all valid neighboring positions (8-directional).
    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        let directions: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dr, dc) in directions {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr >= 0 && nc >= 0 {
                let nr = nr as usize;
                let nc = nc as usize;
                if self.is_valid_position(nr, nc) {
                    neighbors.push((nr, nc));
                }
            }
        }

        neighbors
    }

    /// Reveal a cell. Returns true if the cell was newly revealed.
    pub fn reveal(&mut self, row: usize, col: usize) -> bool {
        if self.is_valid_position(row, col) && !self.revealed[row][col] {
            self.revealed[row][col] = true;
            true
        } else {
            false
        }
    }

    /// Create a board without any eggs (for testing).
    #[cfg(test)]
    pub fn new_empty(width: usize, height: usize) -> Result<Self, BoardError> {
        if width == 0 || height == 0 {
            return Err(BoardError::InvalidDimensions { width, height });
        }

        Ok(Board {
            width,
            height,
            egg_count: 0,
            cells: vec![vec![0; width]; height],
            revealed: vec![vec![false; width]; height],
            eggs: HashSet::new(),
        })
    }

    /// Manually add an egg and recalculate numbers (for testing).
    #[cfg(test)]
    pub fn add_egg(&mut self, row: usize, col: usize) {
        self.eggs.insert((row, col));
        self.cells[row][col] = CellType::Egg.value();
        self.egg_count += 1;
        self.calculate_numbers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = Board::new(5, 5, 3).unwrap();
        assert_eq!(board.width, 5);
        assert_eq!(board.height, 5);
        assert_eq!(board.egg_count, 3);
    }

    #[test]
    fn test_invalid_dimensions() {
        assert!(Board::new(0, 5, 0).is_err());
        assert!(Board::new(5, 0, 0).is_err());
        assert!(Board::new(0, 0, 0).is_err());
    }

    #[test]
    fn test_too_many_eggs() {
        assert!(Board::new(3, 3, 10).is_err()); // 10 eggs in 9 cells
    }

    #[test]
    fn test_cells_initialized() {
        let board = Board::new(5, 5, 2).unwrap();
        assert_eq!(board.cells.len(), 5);
        assert_eq!(board.cells[0].len(), 5);
    }

    #[test]
    fn test_revealed_array_initialized() {
        let board = Board::new(5, 5, 2).unwrap();
        for row in &board.revealed {
            for &cell in row {
                assert!(!cell);
            }
        }
    }

    #[test]
    fn test_egg_placement_count() {
        let board = Board::new(10, 10, 15).unwrap();
        assert_eq!(board.eggs.len(), 15);

        let egg_count = board
            .cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == -1)
            .count();
        assert_eq!(egg_count, 15);
    }

    #[test]
    fn test_is_valid_position() {
        let board = Board::new(5, 5, 2).unwrap();

        assert!(board.is_valid_position(0, 0));
        assert!(board.is_valid_position(4, 4));
        assert!(board.is_valid_position(2, 3));

        // usize can't be negative, so we just test upper bounds
        assert!(!board.is_valid_position(5, 0));
        assert!(!board.is_valid_position(0, 5));
        assert!(!board.is_valid_position(10, 10));
    }

    #[test]
    fn test_get_cell_valid() {
        let board = Board::new_empty(5, 5).unwrap();
        let cell = board.get_cell(0, 0);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap(), 0);
    }

    #[test]
    fn test_get_cell_invalid() {
        let board = Board::new(5, 5, 2).unwrap();
        assert!(board.get_cell(5, 0).is_none());
        assert!(board.get_cell(0, 5).is_none());
    }

    #[test]
    fn test_is_egg() {
        let board = Board::new(5, 5, 3).unwrap();
        for &egg_pos in &board.eggs {
            assert!(board.is_egg(egg_pos.0, egg_pos.1));
        }
    }

    #[test]
    fn test_is_revealed() {
        let mut board = Board::new(5, 5, 2).unwrap();

        assert!(!board.is_revealed(0, 0));

        board.revealed[2][3] = true;
        assert!(board.is_revealed(2, 3));

        assert!(!board.is_revealed(5, 5)); // Out of bounds
    }

    #[test]
    fn test_get_neighbors_corner() {
        let board = Board::new_empty(5, 5).unwrap();

        let neighbors = board.get_neighbors(0, 0);
        assert_eq!(neighbors.len(), 3);

        let expected: HashSet<_> = [(0, 1), (1, 0), (1, 1)].into_iter().collect();
        let actual: HashSet<_> = neighbors.into_iter().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_neighbors_edge() {
        let board = Board::new_empty(5, 5).unwrap();
        let neighbors = board.get_neighbors(0, 2);
        assert_eq!(neighbors.len(), 5);
    }

    #[test]
    fn test_get_neighbors_center() {
        let board = Board::new_empty(5, 5).unwrap();
        let neighbors = board.get_neighbors(2, 2);
        assert_eq!(neighbors.len(), 8);
    }

    #[test]
    fn test_number_calculation_no_eggs() {
        let board = Board::new_empty(5, 5).unwrap();
        for row in &board.cells {
            for &cell in row {
                assert_eq!(cell, 0);
            }
        }
    }

    #[test]
    fn test_number_calculation_with_eggs() {
        let mut board = Board::new_empty(3, 3).unwrap();

        // Place egg at center
        board.add_egg(1, 1);

        // All 8 neighbors should have value 1
        let expected = [[1, 1, 1], [1, -1, 1], [1, 1, 1]];

        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(
                    board.cells[row][col], expected[row][col],
                    "Mismatch at ({}, {})",
                    row, col
                );
            }
        }
    }

    #[test]
    fn test_two_adjacent_eggs() {
        let mut board = Board::new_empty(3, 3).unwrap();

        // Place two adjacent eggs
        board.eggs.insert((1, 0));
        board.eggs.insert((1, 1));
        board.cells[1][0] = CellType::Egg.value();
        board.cells[1][1] = CellType::Egg.value();
        board.egg_count = 2;
        board.calculate_numbers();

        assert_eq!(board.cells[0][0], 2);
        assert_eq!(board.cells[0][1], 2);
        assert_eq!(board.cells[1][2], 1);
    }

    #[test]
    fn test_reveal() {
        let mut board = Board::new_empty(3, 3).unwrap();

        assert!(board.reveal(1, 1)); // First reveal returns true
        assert!(board.revealed[1][1]);

        assert!(!board.reveal(1, 1)); // Second reveal returns false
    }
}
