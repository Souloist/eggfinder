//! Animation state management for TUI effects.

use std::time::Instant;

/// Animation timing constants.
pub struct AnimationConfig;

impl AnimationConfig {
    /// Duration of egg collection flash in milliseconds.
    pub const EGG_FLASH_DURATION_MS: u64 = 500;
    /// Duration of cell reveal animation in milliseconds.
    pub const REVEAL_DURATION_MS: u64 = 250;
    /// Delay per BFS depth level for wave effect in milliseconds.
    pub const WAVE_DELAY_MS: u64 = 50;
}

/// Tracks a single cell's flash animation (e.g., when collecting an egg).
#[derive(Debug, Clone)]
pub struct CellFlash {
    pub row: usize,
    pub col: usize,
    pub start_time: Instant,
}

impl CellFlash {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            start_time: Instant::now(),
        }
    }

    /// Returns animation progress (0.0 to 1.0), or None if animation is complete.
    pub fn progress(&self) -> Option<f32> {
        let elapsed_ms = self.start_time.elapsed().as_millis() as u64;
        if elapsed_ms >= AnimationConfig::EGG_FLASH_DURATION_MS {
            None
        } else {
            Some(elapsed_ms as f32 / AnimationConfig::EGG_FLASH_DURATION_MS as f32)
        }
    }

    /// Returns (row, col, progress) tuple for rendering, or None if complete.
    pub fn render_data(&self) -> Option<(usize, usize, f32)> {
        self.progress().map(|p| (self.row, self.col, p))
    }
}

/// Tracks a cell being revealed as part of a floodfill wave animation.
#[derive(Debug, Clone)]
pub struct RevealAnimation {
    pub row: usize,
    pub col: usize,
    /// BFS depth determines animation delay (wave effect).
    pub depth: usize,
    pub start_time: Instant,
}

impl RevealAnimation {
    pub fn new(row: usize, col: usize, depth: usize, start_time: Instant) -> Self {
        Self {
            row,
            col,
            depth,
            start_time,
        }
    }

    /// Returns animation progress (0.0 to 1.0), accounting for wave delay.
    /// Returns None if animation hasn't started yet (still in delay).
    /// Returns Some(progress) where progress >= 1.0 means animation is complete.
    pub fn progress(&self) -> f32 {
        let delay_ms = (self.depth as u64) * AnimationConfig::WAVE_DELAY_MS;
        let elapsed_ms = self.start_time.elapsed().as_millis() as u64;

        if elapsed_ms < delay_ms {
            // Still waiting for this cell to start
            0.0
        } else {
            let anim_elapsed = elapsed_ms - delay_ms;
            anim_elapsed as f32 / AnimationConfig::REVEAL_DURATION_MS as f32
        }
    }

    /// Returns true if the animation is complete and can be removed.
    pub fn is_complete(&self) -> bool {
        self.progress() >= 1.0
    }

    /// Returns (row, col, progress) tuple for rendering.
    pub fn render_data(&self) -> (usize, usize, f32) {
        (self.row, self.col, self.progress())
    }
}

/// Manages all active animations for the game.
#[derive(Debug, Default)]
pub struct AnimationManager {
    /// Flash animation for egg collection (only one at a time).
    pub egg_flash: Option<CellFlash>,
    /// Wave animations for floodfill reveal.
    pub reveal_animations: Vec<RevealAnimation>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all animations (e.g., when starting a new game).
    pub fn clear(&mut self) {
        self.egg_flash = None;
        self.reveal_animations.clear();
    }

    /// Start a flash animation for an egg collection.
    pub fn start_egg_flash(&mut self, row: usize, col: usize) {
        self.egg_flash = Some(CellFlash::new(row, col));
    }

    /// Start wave animations for revealed cells.
    pub fn start_reveal_wave(&mut self, cells: Vec<(usize, usize, usize)>) {
        let now = Instant::now();
        for (row, col, depth) in cells {
            self.reveal_animations
                .push(RevealAnimation::new(row, col, depth, now));
        }
    }

    /// Update animations, removing completed ones. Call once per frame.
    pub fn update(&mut self) {
        // Clear egg flash if complete
        if let Some(ref flash) = self.egg_flash {
            if flash.progress().is_none() {
                self.egg_flash = None;
            }
        }

        // Remove completed reveal animations
        self.reveal_animations.retain(|anim| !anim.is_complete());
    }

    /// Get flash cell data for rendering.
    pub fn flash_cell(&self) -> Option<(usize, usize, f32)> {
        self.egg_flash.as_ref().and_then(|f| f.render_data())
    }

    /// Get all animating cells for rendering.
    pub fn animating_cells(&self) -> Vec<(usize, usize, f32)> {
        self.reveal_animations
            .iter()
            .filter(|a| !a.is_complete())
            .map(|a| a.render_data())
            .collect()
    }
}
