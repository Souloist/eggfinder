//! EggFinder TUI - Main entry point and event loop.

use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use eggfinder_rs::ui::{self, AppScreen};
use eggfinder_rs::{Board, Difficulty, Direction, GameState};

struct App {
    screen: AppScreen,
    menu_selection: usize,
    difficulty: Option<Difficulty>,
    board: Option<Board>,
    state: Option<GameState>,
}

impl App {
    fn new() -> Self {
        App {
            screen: AppScreen::DifficultySelect,
            menu_selection: 0,
            difficulty: None,
            board: None,
            state: None,
        }
    }

    /// Start a new game with the selected difficulty. Returns true on success.
    fn start_game(&mut self) -> bool {
        let difficulties = [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];
        let diff = difficulties[self.menu_selection];
        let (width, height, eggs) = diff.config();

        match Board::new(width, height, eggs) {
            Ok(board) => {
                self.difficulty = Some(diff);
                self.board = Some(board);
                self.state = Some(GameState::new(width, height));
                self.screen = AppScreen::Playing;
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
    }

    /// Handle revealing a cell at the cursor position.
    fn reveal_at_cursor(&mut self) {
        if let (Some(board), Some(state)) = (&mut self.board, &mut self.state) {
            let (row, col) = state.cursor_position();

            if board.is_revealed(row, col) {
                return;
            }

            board.reveal(row, col);

            if board.is_egg(row, col) {
                state.collect_egg((row, col));
            } else {
                state.use_turn();
                // TODO: Full floodfill for empty cells
            }

            if state.game_over {
                self.screen = AppScreen::GameOver;
            }
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| match app.screen {
            AppScreen::DifficultySelect => {
                ui::render_menu(frame, app.menu_selection);
            }
            AppScreen::Playing => {
                if let (Some(board), Some(state)) = (&app.board, &app.state) {
                    ui::render_game(frame, board, state);
                }
            }
            AppScreen::GameOver => {
                if let (Some(board), Some(state)) = (&app.board, &app.state) {
                    ui::render_game_over(frame, board, state);
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
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
    }
}
