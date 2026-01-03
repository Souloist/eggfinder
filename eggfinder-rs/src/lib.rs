//! EggFinder - A minesweeper-like game where you collect eggs.

pub mod board;
pub mod constants;
pub mod game_state;
pub mod ui;

pub use board::{Board, BoardError};
pub use constants::{CellDisplay, CellType, Difficulty, Direction, GameConfig};
pub use game_state::GameState;
