//! Game constants and configuration.

use ratatui::style::Color;

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
            Difficulty::Easy => (8, 8, 3, 4),
            Difficulty::Medium => (15, 12, 7, 7),
            Difficulty::Hard => (20, 15, 15, 10),
        }
    }

    /// Returns the display name for menu.
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
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
    pub const COLLECTED_EGG: char = '🥚';
    pub const REVEALED_EGG: char = '💩';
}

/// Direction for cursor movement.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// UI color constants.
pub struct UiColors;

impl UiColors {
    /// Bright egg yolk yellow for cursor and animations.
    pub const EGG_YOLK: Color = Color::Rgb(255, 200, 0);
    /// Brown color for unrevealed tiles.
    pub const TILE_BROWN: Color = Color::Rgb(139, 90, 43);
    /// Orange for wave animation start.
    pub const WAVE_ORANGE: Color = Color::Rgb(255, 165, 0);
}
