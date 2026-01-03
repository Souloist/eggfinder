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
}
