//! Game state management.

use std::collections::HashSet;

use crate::board::Coordinate;
use crate::constants::{Direction, GameConfig};

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
    pub fn new(board_width: usize, board_height: usize) -> Self {
        GameState {
            cursor: (0, 0), // Start at top-left
            turns_remaining: GameConfig::DEFAULT_TURNS,
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
    pub fn collect_egg(&mut self, pos: Coordinate) -> bool {
        if self.eggs_collected.insert(pos) {
            self.score += 1;
            self.turns_remaining += GameConfig::EGG_BONUS_TURNS;
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
}
