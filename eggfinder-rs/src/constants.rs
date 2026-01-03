//! Game constants and configuration.

/// Difficulty level for the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// Returns (width, height, egg_count, starting_turns) for this difficulty.
    pub fn config(&self) -> (usize, usize, usize, i32) {
        match self {
            Difficulty::Easy => (4, 4, 3, 4),
            Difficulty::Medium => (7, 7, 7, 7),
            Difficulty::Hard => (10, 10, 15, 10),
        }
    }

    /// Returns the display name for menu.
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy (4x4, 3 eggs, 4 turns)",
            Difficulty::Medium => "Medium (7x7, 7 eggs, 7 turns)",
            Difficulty::Hard => "Hard (10x10, 15 eggs, 10 turns)",
        }
    }
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
    pub const COLLECTED_EGG: char = '🥚'; // Egg emoji for collected
    pub const REVEALED_EGG: char = '💩';  // Poop emoji for missed eggs at game end
    pub const HIDDEN: char = '■';
    pub const EMPTY: char = '·';
}

/// Direction for cursor movement.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
