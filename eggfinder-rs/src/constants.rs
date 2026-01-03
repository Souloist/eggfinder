//! Game constants and configuration.

/// Difficulty level for the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// Returns (width, height, egg_count) for this difficulty.
    pub fn config(&self) -> (usize, usize, usize) {
        match self {
            Difficulty::Easy => (9, 9, 6),
            Difficulty::Medium => (16, 16, 8),
            Difficulty::Hard => (25, 25, 10),
        }
    }

    /// Returns the display name for menu.
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy (9x9, 6 eggs)",
            Difficulty::Medium => "Medium (16x16, 8 eggs)",
            Difficulty::Hard => "Hard (25x25, 10 eggs)",
        }
    }
}

/// Game rule configuration constants.
pub struct GameConfig;

impl GameConfig {
    pub const DEFAULT_TURNS: i32 = 10;
    pub const EGG_BONUS_TURNS: i32 = 1;
}

/// Cell type constants for board representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Egg,
    Empty,
}

impl CellType {
    /// Convert to the numeric value used in the cells grid.
    pub fn value(&self) -> i8 {
        match self {
            CellType::Egg => -1,
            CellType::Empty => 0,
        }
    }
}

/// Display symbols for board cells.
pub struct CellDisplay;

impl CellDisplay {
    pub const COLLECTED_EGG: char = '★';
    pub const REVEALED_EGG: char = '○';
    pub const HIDDEN: char = '■';
    pub const EMPTY: char = '·';
    pub const CURSOR: char = '█'; // New: cursor indicator
}

/// Direction for cursor movement.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
