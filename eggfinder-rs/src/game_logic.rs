//! Game logic including floodfill reveal.

use std::collections::{HashSet, VecDeque};

use crate::board::Board;
use crate::constants::CellType;

/// Result of a floodfill reveal operation.
/// Contains cells with their BFS depth (distance from starting cell).
pub struct FloodfillResult {
    /// Cells revealed with their distance from the starting point: (row, col, depth)
    pub cells: Vec<(usize, usize, usize)>,
}

impl FloodfillResult {
    /// Returns the number of cells revealed.
    pub fn count(&self) -> usize {
        self.cells.len()
    }
}

/// BFS floodfill to reveal connected empty cells and border numbers.
/// Eggs are never revealed (matching minesweeper behavior).
/// Returns cells with their BFS depth for wave animation.
pub fn floodfill_reveal(board: &mut Board, row: usize, col: usize) -> FloodfillResult {
    let mut queue = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cells = Vec::new();

    // Queue contains (row, col, depth)
    queue.push_back((row, col, 0usize));

    while let Some((r, c, depth)) = queue.pop_front() {
        if visited.contains(&(r, c)) {
            continue;
        }

        if !board.is_valid_position(r, c) {
            continue;
        }

        if board.revealed[r][c] {
            continue;
        }

        if board.is_egg(r, c) {
            continue;
        }

        visited.insert((r, c));
        board.revealed[r][c] = true;
        cells.push((r, c, depth));

        let cell_value = board.cells[r][c];

        // Only expand from empty cells (value 0)
        if cell_value == CellType::Empty.value() {
            for (nr, nc) in board.get_neighbors(r, c) {
                if !visited.contains(&(nr, nc)) {
                    queue.push_back((nr, nc, depth + 1));
                }
            }
        }
    }

    FloodfillResult { cells }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floodfill_single_cell_with_number() {
        // Board with an egg at center - clicking corner reveals only 1 cell (a number)
        let mut board = Board::new_empty(3, 3).unwrap();
        board.add_egg(1, 1);

        let result = floodfill_reveal(&mut board, 0, 0);
        assert_eq!(result.count(), 1);
        assert!(board.revealed[0][0]);
    }

    #[test]
    fn test_floodfill_empty_board() {
        // Empty board - floodfill reveals everything
        let mut board = Board::new_empty(5, 5).unwrap();

        let result = floodfill_reveal(&mut board, 0, 0);
        assert_eq!(result.count(), 25);

        for row in &board.revealed {
            for &cell in row {
                assert!(cell);
            }
        }
    }

    #[test]
    fn test_floodfill_stops_at_numbers() {
        // Board with egg at corner - floodfill should reveal most cells
        let mut board = Board::new_empty(5, 5).unwrap();
        board.add_egg(0, 0);

        let result = floodfill_reveal(&mut board, 4, 4);

        assert!(result.count() > 1);
        assert!(board.revealed[4][4]);
    }

    #[test]
    fn test_floodfill_already_revealed() {
        let mut board = Board::new_empty(3, 3).unwrap();
        board.revealed[1][1] = true;

        let result = floodfill_reveal(&mut board, 1, 1);
        assert_eq!(result.count(), 0);
    }

    #[test]
    fn test_floodfill_does_not_reveal_eggs() {
        let mut board = Board::new_empty(5, 5).unwrap();
        board.add_egg(2, 2);

        floodfill_reveal(&mut board, 0, 0);

        // Egg should not be revealed
        assert!(!board.revealed[2][2]);
    }

    #[test]
    fn test_floodfill_reveals_numbers_but_stops() {
        // When floodfill hits a number, it reveals it but doesn't expand through it
        let mut board = Board::new_empty(5, 5).unwrap();
        board.add_egg(2, 2);

        floodfill_reveal(&mut board, 0, 0);

        // Numbers around the egg should be revealed
        assert!(board.revealed[1][1]); // This is a number (1)
        assert!(board.revealed[1][2]); // This is a number (1)

        // But egg itself should not be revealed
        assert!(!board.revealed[2][2]);
    }
}
