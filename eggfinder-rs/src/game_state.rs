//! Game state management.

use std::collections::HashSet;

use crate::board::Coordinate;
use crate::constants::{Difficulty, Direction};

/// Tracks current game progress and TUI cursor position.
#[derive(Debug)]
pub struct GameState {
    pub cursor: Coordinate,
    pub turns_remaining: i32,
    pub eggs_collected: HashSet<Coordinate>,
    pub game_over: bool,
    pub score: u32,
    pub difficulty: Difficulty,
    board_width: usize,
    board_height: usize,
}

impl GameState {
    pub fn new(difficulty: Difficulty) -> Self {
        let (width, height, _, turns) = difficulty.config();
        GameState {
            cursor: (0, 0), // Start at top-left
            turns_remaining: turns,
            eggs_collected: HashSet::new(),
            game_over: false,
            score: 0,
            difficulty,
            board_width: width,
            board_height: height,
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
        let (_, _, _, expected_turns) = Difficulty::Easy.config();
        let state = GameState::new(Difficulty::Easy);

        assert_eq!(state.cursor, (0, 0));
        assert_eq!(state.turns_remaining, expected_turns);
        assert!(state.eggs_collected.is_empty());
        assert!(!state.game_over);
        assert_eq!(state.score, 0);
        assert_eq!(state.difficulty, Difficulty::Easy);
    }

    #[test]
    fn test_collect_egg_does_not_affect_turns() {
        let mut state = GameState::new(Difficulty::Easy);
        let initial_turns = state.turns_remaining;

        state.collect_egg((0, 0));

        assert_eq!(state.turns_remaining, initial_turns);
        assert_eq!(state.eggs_collected.len(), 1);
        assert_eq!(state.score, 1);
    }

    #[test]
    fn test_collect_egg_returns_false_for_duplicate() {
        let mut state = GameState::new(Difficulty::Easy);

        assert!(state.collect_egg((0, 0))); // First collection
        assert!(!state.collect_egg((0, 0))); // Duplicate

        assert_eq!(state.eggs_collected.len(), 1);
        assert_eq!(state.score, 1); // Score doesn't increase for duplicate
    }

    #[test]
    fn test_use_turn_decrements_turns() {
        let mut state = GameState::new(Difficulty::Medium); // 7 turns
        let initial_turns = state.turns_remaining;

        state.use_turn();

        assert_eq!(state.turns_remaining, initial_turns - 1);
        assert!(!state.game_over);
    }

    #[test]
    fn test_use_turn_triggers_game_over_at_zero() {
        let (_, _, _, starting_turns) = Difficulty::Easy.config();
        let mut state = GameState::new(Difficulty::Easy);

        // Use all turns
        for _ in 0..starting_turns {
            assert!(!state.game_over);
            state.use_turn();
        }

        assert_eq!(state.turns_remaining, 0);
        assert!(state.game_over);
    }

    #[test]
    fn test_all_eggs_collected() {
        let mut state = GameState::new(Difficulty::Easy);

        state.collect_egg((0, 0));
        state.collect_egg((1, 1));
        state.collect_egg((2, 2));

        assert!(!state.all_eggs_collected(5)); // 3 of 5
        assert!(state.all_eggs_collected(3)); // 3 of 3
    }

    #[test]
    fn test_cursor_movement_respects_bounds() {
        let (width, height, _, _) = Difficulty::Easy.config();
        let mut state = GameState::new(Difficulty::Easy);
        let max_row = height - 1;
        let max_col = width - 1;

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
        for _ in 0..(max_row.max(max_col)) {
            state.move_cursor(Direction::Down);
            state.move_cursor(Direction::Right);
        }
        assert_eq!(state.cursor, (max_row, max_col));

        // Can't go beyond bounds
        state.move_cursor(Direction::Down);
        assert_eq!(state.cursor, (max_row, max_col));
        state.move_cursor(Direction::Right);
        assert_eq!(state.cursor, (max_row, max_col));
    }

    #[test]
    fn test_is_cursor_at() {
        let mut state = GameState::new(Difficulty::Easy);

        assert!(state.is_cursor_at(0, 0));
        assert!(!state.is_cursor_at(1, 1));

        state.move_cursor(Direction::Down);
        state.move_cursor(Direction::Right);

        assert!(!state.is_cursor_at(0, 0));
        assert!(state.is_cursor_at(1, 1));
    }
}
