//! Game state management.

use std::collections::HashSet;

use crate::board::Coordinate;
use crate::constants::Direction;

/// Tracks current game progress and TUI cursor position.
#[derive(Debug)]
pub struct GameState {
    pub cursor: Coordinate,
    pub turns_remaining: i32,
    pub eggs_collected: HashSet<Coordinate>,
    pub game_over: bool,
    pub score: u32,
    board_width: usize,
    board_height: usize,
}

impl GameState {
    pub fn new(board_width: usize, board_height: usize, starting_turns: i32) -> Self {
        GameState {
            cursor: (0, 0), // Start at top-left
            turns_remaining: starting_turns,
            eggs_collected: HashSet::new(),
            game_over: false,
            score: 0,
            board_width,
            board_height,
        }
    }

    /// Move the cursor, respecting board bounds.
    pub fn move_cursor(&mut self, direction: Direction) {
        let (row, col) = self.cursor;

        self.cursor = match direction {
            Direction::Up => {
                if row > 0 {
                    (row - 1, col)
                } else {
                    (row, col)
                }
            }
            Direction::Down => {
                if row + 1 < self.board_height {
                    (row + 1, col)
                } else {
                    (row, col)
                }
            }
            Direction::Left => {
                if col > 0 {
                    (row, col - 1)
                } else {
                    (row, col)
                }
            }
            Direction::Right => {
                if col + 1 < self.board_width {
                    (row, col + 1)
                } else {
                    (row, col)
                }
            }
        };
    }

    pub fn cursor_position(&self) -> Coordinate {
        self.cursor
    }

    pub fn is_cursor_at(&self, row: usize, col: usize) -> bool {
        self.cursor == (row, col)
    }

    /// Collect an egg and update score. Returns true if newly collected.
    /// Note: Collecting eggs does not affect turn count.
    pub fn collect_egg(&mut self, pos: Coordinate) -> bool {
        if self.eggs_collected.insert(pos) {
            self.score += 1;
            true
        } else {
            false
        }
    }

    /// Use a turn. Triggers game over if turns reach 0.
    pub fn use_turn(&mut self) {
        self.turns_remaining -= 1;
        if self.turns_remaining <= 0 {
            self.game_over = true;
        }
    }

    /// Check if all eggs have been collected.
    /// Note: The win condition check (setting game_over) is handled by the caller.
    pub fn all_eggs_collected(&self, total_eggs: usize) -> bool {
        self.eggs_collected.len() == total_eggs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_state() {
        let state = GameState::new(5, 5, 10);
        assert_eq!(state.cursor, (0, 0));
        assert_eq!(state.turns_remaining, 10);
        assert!(state.eggs_collected.is_empty());
        assert!(!state.game_over);
        assert_eq!(state.score, 0);
    }

    #[test]
    fn test_collect_egg_does_not_affect_turns() {
        let mut state = GameState::new(5, 5, 10);
        let initial_turns = state.turns_remaining;

        state.collect_egg((0, 0));

        assert_eq!(state.turns_remaining, initial_turns);
        assert_eq!(state.eggs_collected.len(), 1);
        assert_eq!(state.score, 1);
    }

    #[test]
    fn test_collect_egg_returns_false_for_duplicate() {
        let mut state = GameState::new(5, 5, 10);

        assert!(state.collect_egg((0, 0))); // First collection
        assert!(!state.collect_egg((0, 0))); // Duplicate

        assert_eq!(state.eggs_collected.len(), 1);
        assert_eq!(state.score, 1); // Score doesn't increase for duplicate
    }

    #[test]
    fn test_use_turn_decrements_turns() {
        let mut state = GameState::new(5, 5, 10);

        state.use_turn();

        assert_eq!(state.turns_remaining, 9);
        assert!(!state.game_over);
    }

    #[test]
    fn test_use_turn_triggers_game_over_at_zero() {
        let mut state = GameState::new(5, 5, 1); // Start with 1 turn

        assert!(!state.game_over);
        state.use_turn();

        assert_eq!(state.turns_remaining, 0);
        assert!(state.game_over);
    }

    #[test]
    fn test_all_eggs_collected() {
        let mut state = GameState::new(5, 5, 10);

        state.collect_egg((0, 0));
        state.collect_egg((1, 1));
        state.collect_egg((2, 2));

        assert!(!state.all_eggs_collected(5)); // 3 of 5
        assert!(state.all_eggs_collected(3)); // 3 of 3
    }

    #[test]
    fn test_cursor_movement_respects_bounds() {
        let mut state = GameState::new(3, 3, 10);

        // At (0,0), can't go up or left
        state.move_cursor(Direction::Up);
        assert_eq!(state.cursor, (0, 0));
        state.move_cursor(Direction::Left);
        assert_eq!(state.cursor, (0, 0));

        // Can go down and right
        state.move_cursor(Direction::Down);
        assert_eq!(state.cursor, (1, 0));
        state.move_cursor(Direction::Right);
        assert_eq!(state.cursor, (1, 1));

        // Move to bottom-right corner
        state.move_cursor(Direction::Down);
        state.move_cursor(Direction::Right);
        assert_eq!(state.cursor, (2, 2));

        // Can't go beyond bounds
        state.move_cursor(Direction::Down);
        assert_eq!(state.cursor, (2, 2));
        state.move_cursor(Direction::Right);
        assert_eq!(state.cursor, (2, 2));
    }

    #[test]
    fn test_is_cursor_at() {
        let mut state = GameState::new(5, 5, 10);

        assert!(state.is_cursor_at(0, 0));
        assert!(!state.is_cursor_at(1, 1));

        state.move_cursor(Direction::Down);
        state.move_cursor(Direction::Right);

        assert!(!state.is_cursor_at(0, 0));
        assert!(state.is_cursor_at(1, 1));
    }
}
