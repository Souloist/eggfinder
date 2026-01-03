//! UI rendering using ratatui with bordered cells and egg theme.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::board::Board;
use crate::constants::{CellDisplay, CellType, Difficulty};
use crate::game_state::GameState;

/// Egg-themed yellow color for borders.
const EGG_YELLOW: Color = Color::Yellow;
const EGG_BORDER_STYLE: Style = Style::new().fg(EGG_YELLOW);

/// Cell dimensions for the grid (5x3 looks more square and fits 2-wide emojis).
const CELL_WIDTH: u16 = 5;
const CELL_HEIGHT: u16 = 3;

/// Current screen in the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    DifficultySelect,
    Playing,
    GameOver,
}

/// Render the difficulty selection menu.
pub fn render_menu(frame: &mut Frame, selected: usize) {
    let area = frame.area();

    let difficulties = [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "🥚 EggFinder 🥚",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Select Difficulty:"),
        Line::from(""),
    ];

    for (i, diff) in difficulties.iter().enumerate() {
        let style = if i == selected {
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let prefix = if i == selected { "> " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, diff.name()),
            style,
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Use W/S to navigate, SPACE to select, Q to quit",
        Style::default().fg(Color::DarkGray),
    )));

    let menu = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(EGG_BORDER_STYLE)
                .border_type(BorderType::Rounded)
                .title("🥚 EggFinder"),
        )
        .alignment(Alignment::Center);

    frame.render_widget(menu, area);
}

/// Render the game board, status bar, and egg counter.
/// `flash_cell` is an optional (row, col, progress) where progress is 0.0-1.0 for flash animation.
pub fn render_game(
    frame: &mut Frame,
    board: &Board,
    state: &GameState,
    flash_cell: Option<(usize, usize, f32)>,
) {
    let area = frame.area();

    // Layout: Status bar on top, then board + side panel
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    render_status(frame, main_chunks[0], board, state);

    // Split bottom area: board on left, egg counter on right
    let game_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(22)])
        .split(main_chunks[1]);

    render_board(frame, game_chunks[0], board, state, flash_cell);
    render_egg_counter(frame, game_chunks[1], board, state);
}

fn render_status(frame: &mut Frame, area: Rect, board: &Board, state: &GameState) {
    let status_text = format!(
        "Turns: {} | WASD: move | SPACE: reveal | Q: quit",
        state.turns_remaining
    );

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(EGG_BORDER_STYLE)
                .border_type(BorderType::Rounded)
                .title(format!(
                    "🥚 EggFinder - {} Mode",
                    get_difficulty_name(board)
                )),
        );

    frame.render_widget(status, area);
}

fn get_difficulty_name(board: &Board) -> &'static str {
    match (board.width, board.height) {
        (9, 9) => "Easy",
        (16, 16) => "Medium",
        (25, 25) => "Hard",
        _ => "Custom",
    }
}

fn render_board(
    frame: &mut Frame,
    area: Rect,
    board: &Board,
    state: &GameState,
    flash_cell: Option<(usize, usize, f32)>,
) {
    // Create outer block with yellow border and title (fills available area)
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(EGG_BORDER_STYLE)
        .border_type(BorderType::Rounded)
        .title("🥚 Board");

    let outer_inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    // Calculate grid dimensions (for inner border sizing)
    let grid_width = board.width as u16 * CELL_WIDTH;
    let grid_height = board.height as u16 * CELL_HEIGHT;

    // Inner border is tight around the grid, centered in outer area
    let inner_width = grid_width + 2; // +2 for inner border
    let inner_height = grid_height + 2;
    let inner_area = center_rect(outer_inner, inner_width, inner_height);

    // Add inner boundary (tight border around the grid)
    let inner_block = Block::default()
        .borders(Borders::ALL)
        .border_style(EGG_BORDER_STYLE)
        .border_type(BorderType::Thick);

    let grid_area = inner_block.inner(inner_area);
    frame.render_widget(inner_block, inner_area);

    // Create row constraints
    let row_constraints: Vec<Constraint> = (0..board.height)
        .map(|_| Constraint::Length(CELL_HEIGHT))
        .collect();

    let row_rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(grid_area);

    // Render each row of cells
    for (row, row_rect) in row_rects.iter().enumerate() {
        let col_constraints: Vec<Constraint> = (0..board.width)
            .map(|_| Constraint::Length(CELL_WIDTH))
            .collect();

        let col_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(*row_rect);

        for (col, cell_rect) in col_rects.iter().enumerate() {
            // Check if this cell should flash
            let is_flashing = flash_cell
                .map(|(fr, fc, _)| fr == row && fc == col)
                .unwrap_or(false);
            let flash_progress = if is_flashing {
                flash_cell.map(|(_, _, p)| p).unwrap_or(1.0)
            } else {
                1.0
            };

            render_cell(frame, *cell_rect, board, state, row, col, is_flashing, flash_progress);
        }
    }
}

fn render_cell(
    frame: &mut Frame,
    area: Rect,
    board: &Board,
    state: &GameState,
    row: usize,
    col: usize,
    is_flashing: bool,
    flash_progress: f32,
) {
    let is_cursor = state.is_cursor_at(row, col);
    let (content, content_style) = get_cell_content(board, state, row, col);

    // Determine cell style and border color
    let (cell_style, border_color) = if is_flashing && flash_progress < 1.0 {
        // Flashing cell: bright yellow border that fades
        let intensity = 1.0 - flash_progress;
        let border_color = if intensity > 0.5 {
            Color::LightYellow
        } else {
            Color::Yellow
        };
        (content_style, border_color)
    } else if is_cursor {
        // Cursor cell: yellow border only (no background change)
        (content_style, Color::Yellow)
    } else {
        (content_style, Color::DarkGray)
    };

    let cell = Paragraph::new(content)
        .alignment(Alignment::Center)
        .style(cell_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(border_color)),
        );

    frame.render_widget(cell, area);
}

/// Get the content character and style for a cell.
fn get_cell_content(board: &Board, state: &GameState, row: usize, col: usize) -> (String, Style) {
    let is_revealed = board.is_revealed(row, col);
    let is_egg = board.is_egg(row, col);
    let is_collected = state.eggs_collected.contains(&(row, col));

    if !is_revealed {
        (
            CellDisplay::HIDDEN.to_string(),
            Style::default().fg(Color::Blue),
        )
    } else if is_egg {
        if is_collected {
            (
                CellDisplay::COLLECTED_EGG.to_string(),
                Style::default().fg(Color::Green),
            )
        } else {
            (
                CellDisplay::REVEALED_EGG.to_string(),
                Style::default().fg(Color::Yellow),
            )
        }
    } else {
        let cell_value = board.cells[row][col];
        if cell_value == CellType::Empty.value() {
            // Empty cells show nothing (blank)
            (" ".to_string(), Style::default())
        } else {
            // All numbers are white for clarity against black background
            let ch = char::from_digit(cell_value as u32, 10).unwrap_or('?');
            (ch.to_string(), Style::default().fg(Color::White))
        }
    }
}

fn render_egg_counter(frame: &mut Frame, area: Rect, board: &Board, state: &GameState) {
    let eggs = state.eggs_collected.len();
    let total = board.egg_count;
    let phrase = get_inflation_phrase(eggs, total, state.game_over);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "🥚 Eggs Collected",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("{} / {}", eggs, total),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            phrase,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let widget = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(EGG_BORDER_STYLE)
            .border_type(BorderType::Rounded)
            .title("💰 Savings"),
    );

    frame.render_widget(widget, area);
}

fn get_inflation_phrase(eggs: usize, total: usize, game_over: bool) -> &'static str {
    if game_over && eggs == 0 {
        "Inflation won 📈"
    } else if game_over && eggs == total {
        "You beat inflation! 🎉"
    } else if game_over {
        "Could've been worse..."
    } else {
        match eggs {
            0 => "Eggs are $7 each now",
            1 => "You saved $7!",
            2 => "That's $14 saved!",
            3 => "Wow, $21 in savings!",
            4 => "$28! Take that!",
            5 => "$35! Egg-cellent!",
            6 => "$42! On a roll!",
            7 => "$49! Incredible!",
            8 => "$56! Unstoppable!",
            9 => "$63! Legend!",
            _ => "Inflation destroyer! 💪",
        }
    }
}

/// Create a centered rectangle within the given area.
fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    Rect::new(x, y, width, height)
}

/// Render the game over screen.
pub fn render_game_over(frame: &mut Frame, board: &Board, state: &GameState) {
    let area = frame.area();

    let eggs = state.eggs_collected.len();
    let total = board.egg_count;

    let (message, message_color) = if eggs == total {
        ("🎉 PERFECT! You beat inflation! 🎉", Color::Green)
    } else if eggs == 0 {
        ("💸 Inflation won this time... 📈", Color::Red)
    } else {
        ("Game Over!", Color::Yellow)
    };

    let savings = eggs * 7;

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default()
                .fg(message_color)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("Eggs Collected: {} / {}", eggs, total)),
        Line::from(format!("Total Savings: ${}", savings)),
        Line::from(""),
        Line::from(Span::styled(
            get_inflation_phrase(eggs, total, true),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press R to restart or Q to quit",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let game_over = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(EGG_BORDER_STYLE)
                .border_type(BorderType::Rounded)
                .title("🥚 Game Over"),
        )
        .alignment(Alignment::Center);

    frame.render_widget(game_over, area);
}
