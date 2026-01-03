//! EggFinder TUI - Main entry point and event loop with animations.

use std::io;
use std::panic;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

/// Install a panic hook that restores the terminal before printing the panic message.
/// This ensures the terminal is usable even if the app crashes.
fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Attempt to restore terminal state
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        // Call the original panic hook to print the error
        original_hook(panic_info);
    }));
}
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::Block,
    Terminal,
};
use tachyonfx::EffectManager;

use eggfinder_rs::ui::{self, AppScreen};
use eggfinder_rs::{floodfill_reveal, Board, Difficulty, Direction, GameState};

/// Animation duration constants
const EGG_FLASH_DURATION_MS: u64 = 500;
const REVEAL_ANIMATION_DURATION_MS: u64 = 250;
const REVEAL_WAVE_DELAY_MS: u64 = 50; // Delay per BFS depth level

struct App {
    screen: AppScreen,
    menu_selection: usize,
    difficulty: Option<Difficulty>,
    board: Option<Board>,
    state: Option<GameState>,
    effects: EffectManager<()>,
    last_frame: Instant,
    /// Cell that should flash (row, col, start_time) for egg collection
    egg_flash: Option<(usize, usize, Instant)>,
    /// Cells animating from floodfill reveal: (row, col, depth, start_time)
    reveal_animations: Vec<(usize, usize, usize, Instant)>,
}

impl App {
    fn new() -> Self {
        App {
            screen: AppScreen::DifficultySelect,
            menu_selection: 0,
            difficulty: None,
            board: None,
            state: None,
            effects: EffectManager::default(),
            last_frame: Instant::now(),
            egg_flash: None,
            reveal_animations: Vec::new(),
        }
    }

    /// Start a new game with the selected difficulty. Returns true on success.
    fn start_game(&mut self) -> bool {
        let difficulties = [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];
        let diff = difficulties[self.menu_selection];
        let (width, height, eggs, turns) = diff.config();

        match Board::new(width, height, eggs) {
            Ok(board) => {
                self.difficulty = Some(diff);
                self.board = Some(board);
                self.state = Some(GameState::new(width, height, turns));
                self.screen = AppScreen::Playing;
                self.effects = EffectManager::default();
                self.egg_flash = None;
                self.reveal_animations.clear();
                true
            }
            Err(_) => false,
        }
    }

    fn restart(&mut self) {
        self.screen = AppScreen::DifficultySelect;
        self.menu_selection = 0;
        self.difficulty = None;
        self.board = None;
        self.state = None;
        self.effects = EffectManager::default();
        self.egg_flash = None;
        self.reveal_animations.clear();
    }

    /// Handle revealing a cell at the cursor position.
    fn reveal_at_cursor(&mut self) {
        // Track egg position for flash effect
        let mut found_egg_at: Option<(usize, usize)> = None;
        // Track revealed cells for wave animation
        let mut revealed_cells: Vec<(usize, usize, usize)> = Vec::new();

        if let (Some(board), Some(state)) = (&mut self.board, &mut self.state) {
            let (row, col) = state.cursor_position();

            if board.is_revealed(row, col) {
                return;
            }

            if board.is_egg(row, col) {
                // Egg found! Reveal and collect it
                board.reveal(row, col);
                state.collect_egg((row, col));
                found_egg_at = Some((row, col));

                // Check win condition: all eggs collected
                if state.all_eggs_collected(board.egg_count) {
                    state.game_over = true;
                }
            } else {
                // Normal cell - use floodfill to reveal connected empty cells
                let result = floodfill_reveal(board, row, col);
                revealed_cells = result.cells;
                state.use_turn();
            }

            // Check if game ended (win or lose)
            if state.game_over {
                self.screen = AppScreen::GameOver;
            }
        }

        // Trigger cell-specific flash for egg collection
        if let Some((row, col)) = found_egg_at {
            self.egg_flash = Some((row, col, Instant::now()));
        }

        // Start wave animation for revealed cells
        if !revealed_cells.is_empty() {
            let now = Instant::now();
            for (row, col, depth) in revealed_cells {
                self.reveal_animations.push((row, col, depth, now));
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Install panic hook before entering raw mode
    install_panic_hook();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal state on normal exit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        // Calculate delta time for animations
        let elapsed = app.last_frame.elapsed();
        app.last_frame = Instant::now();

        // Calculate flash cell progress (0.0 = start, 1.0 = done)
        let flash_cell = app.egg_flash.and_then(|(row, col, start_time)| {
            let elapsed_ms = start_time.elapsed().as_millis() as u64;
            if elapsed_ms >= EGG_FLASH_DURATION_MS {
                None // Animation done
            } else {
                let progress = elapsed_ms as f32 / EGG_FLASH_DURATION_MS as f32;
                Some((row, col, progress))
            }
        });

        // Clear flash if animation is done
        if app.egg_flash.is_some() && flash_cell.is_none() {
            app.egg_flash = None;
        }

        // Calculate reveal animation progress for each cell: (row, col, progress 0.0-1.0)
        let mut animating_cells: Vec<(usize, usize, f32)> = Vec::new();
        let mut finished_indices: Vec<usize> = Vec::new();

        for (i, (row, col, depth, start_time)) in app.reveal_animations.iter().enumerate() {
            let delay_ms = (*depth as u64) * REVEAL_WAVE_DELAY_MS;
            let elapsed_ms = start_time.elapsed().as_millis() as u64;

            if elapsed_ms < delay_ms {
                // Still waiting for this cell to start
                animating_cells.push((*row, *col, 0.0));
            } else {
                let anim_elapsed = elapsed_ms - delay_ms;
                if anim_elapsed >= REVEAL_ANIMATION_DURATION_MS {
                    // Animation finished for this cell
                    finished_indices.push(i);
                } else {
                    let progress = anim_elapsed as f32 / REVEAL_ANIMATION_DURATION_MS as f32;
                    animating_cells.push((*row, *col, progress));
                }
            }
        }

        // Remove finished animations (reverse order to preserve indices)
        for i in finished_indices.into_iter().rev() {
            app.reveal_animations.remove(i);
        }

        terminal.draw(|frame| {
            let area = frame.area();

            // Force black background for consistent appearance across terminal themes
            let bg = Block::default().style(Style::default().bg(Color::Black));
            frame.render_widget(bg, area);

            match app.screen {
                AppScreen::DifficultySelect => {
                    ui::render_menu(frame, app.menu_selection);
                }
                AppScreen::Playing => {
                    if let (Some(board), Some(state)) = (&app.board, &app.state) {
                        ui::render_game(frame, board, state, flash_cell, &animating_cells);
                    }
                }
                AppScreen::GameOver => {
                    if let (Some(board), Some(state)) = (&app.board, &app.state) {
                        ui::render_game_over(frame, board, state);
                    }
                }
            }

            // Apply tachyonfx effects
            app.effects
                .process_effects(elapsed.into(), frame.buffer_mut(), area);
        })?;

        // Poll for events with a short timeout to keep animations smooth
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                // Handle terminal resize - just continue to redraw
                Event::Resize(_, _) => continue,

                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    match app.screen {
                        AppScreen::DifficultySelect => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char('w') | KeyCode::Up => {
                                if app.menu_selection > 0 {
                                    app.menu_selection -= 1;
                                }
                            }
                            KeyCode::Char('s') | KeyCode::Down => {
                                if app.menu_selection < 2 {
                                    app.menu_selection += 1;
                                }
                            }
                            KeyCode::Char(' ') | KeyCode::Enter => {
                                let _ = app.start_game();
                            }
                            _ => {}
                        },
                        AppScreen::Playing => {
                            if let Some(state) = &mut app.state {
                                match key.code {
                                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                                    KeyCode::Char('b') => {
                                        app.restart(); // Go back to difficulty selection
                                    }
                                    KeyCode::Char('w') | KeyCode::Up => {
                                        state.move_cursor(Direction::Up);
                                    }
                                    KeyCode::Char('s') | KeyCode::Down => {
                                        state.move_cursor(Direction::Down);
                                    }
                                    KeyCode::Char('a') | KeyCode::Left => {
                                        state.move_cursor(Direction::Left);
                                    }
                                    KeyCode::Char('d') | KeyCode::Right => {
                                        state.move_cursor(Direction::Right);
                                    }
                                    KeyCode::Char(' ') | KeyCode::Enter => {
                                        app.reveal_at_cursor();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        AppScreen::GameOver => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char('r') => {
                                app.restart();
                            }
                            _ => {}
                        },
                    }
                }

                // Ignore other events (mouse, focus, paste, etc.)
                _ => {}
            }
        }
    }
}
