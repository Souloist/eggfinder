//! EggFinder TUI - Main entry point and event loop with animations.

use std::io;
use std::panic;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::Block,
    Terminal,
};

use eggfinder_rs::ui::{self, AppScreen};
use eggfinder_rs::{floodfill_reveal, AnimationManager, Board, Difficulty, Direction, GameState};

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

struct App {
    screen: AppScreen,
    menu_selection: usize,
    difficulty: Option<Difficulty>,
    board: Option<Board>,
    state: Option<GameState>,
    animations: AnimationManager,
}

impl App {
    fn new() -> Self {
        App {
            screen: AppScreen::DifficultySelect,
            menu_selection: 0,
            difficulty: None,
            board: None,
            state: None,
            animations: AnimationManager::new(),
        }
    }

    /// Start a new game with the selected difficulty. Returns true on success.
    fn start_game(&mut self) -> bool {
        let difficulties = [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];
        let diff = difficulties[self.menu_selection];
        let (width, height, eggs, _) = diff.config();

        match Board::new(width, height, eggs) {
            Ok(board) => {
                self.difficulty = Some(diff);
                self.board = Some(board);
                self.state = Some(GameState::new(diff));
                self.screen = AppScreen::Playing;
                self.animations.clear();
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
        self.animations.clear();
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
            self.animations.start_egg_flash(row, col);
        }

        // Start wave animation for revealed cells
        if !revealed_cells.is_empty() {
            self.animations.start_reveal_wave(revealed_cells);
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
        // Update animations (cleans up completed ones)
        app.animations.update();

        // Get animation data for rendering
        let flash_cell = app.animations.flash_cell();
        let animating_cells = app.animations.animating_cells();

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
