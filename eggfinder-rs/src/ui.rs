//! UI rendering using ratatui with bordered cells and egg theme.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
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
/// `flash_cell` is an optional (row, col, progress) for egg flash animation.
/// `animating_cells` contains cells being revealed with wave animation: (row, col, progress).
pub fn render_game(
    frame: &mut Frame,
    board: &Board,
    state: &GameState,
    flash_cell: Option<(usize, usize, f32)>,
    animating_cells: &[(usize, usize, f32)],
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

    render_board(frame, game_chunks[0], board, state, flash_cell, animating_cells);
    render_egg_counter(frame, game_chunks[1], board, state);
}

fn render_status(frame: &mut Frame, area: Rect, board: &Board, state: &GameState) {
    let status_text = format!(
        "Turns: {} | WASD: move | SPACE: reveal | B: back | Q: quit",
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
    animating_cells: &[(usize, usize, f32)],
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

    // Add inner boundary (tight border around the grid) with rounded edges
    let inner_block = Block::default()
        .borders(Borders::ALL)
        .border_style(EGG_BORDER_STYLE)
        .border_type(BorderType::Rounded);

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
            // Check if this cell should flash (egg collection)
            let is_flashing = flash_cell
                .map(|(fr, fc, _)| fr == row && fc == col)
                .unwrap_or(false);
            let flash_progress = if is_flashing {
                flash_cell.map(|(_, _, p)| p).unwrap_or(1.0)
            } else {
                1.0
            };

            // Check if this cell is animating (wave reveal)
            let reveal_progress = animating_cells
                .iter()
                .find(|(r, c, _)| *r == row && *c == col)
                .map(|(_, _, p)| *p);

            render_cell(
                frame,
                *cell_rect,
                board,
                state,
                row,
                col,
                is_flashing,
                flash_progress,
                reveal_progress,
            );
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
    reveal_progress: Option<f32>,
) {
    let is_cursor = state.is_cursor_at(row, col);
    let is_revealed = board.is_revealed(row, col);
    let (content, content_style) = get_cell_content(board, state, row, col);

    // Bright egg yolk yellow for cursor and animations
    let egg_yolk = Color::Rgb(255, 200, 0);

    // Determine cell style, border color, and optional inner background
    let (cell_style, border_color, inner_bg) = if !is_revealed {
        // Unrevealed cell: brown background with yellow border
        let brown = Color::Rgb(139, 90, 43); // Saddle brown
        if is_cursor {
            // Cursor on unrevealed: bright egg yolk border
            (Style::default(), egg_yolk, Some(brown))
        } else {
            (Style::default(), Color::Yellow, Some(brown))
        }
    } else if let Some(progress) = reveal_progress {
        // Wave reveal animation: transition from egg yolk to yellow to normal
        if progress < 1.0 {
            // Interpolate border color from egg yolk to yellow to dark gray
            let border_color = if progress < 0.3 {
                egg_yolk // Bright egg yolk yellow
            } else if progress < 0.6 {
                Color::Yellow
            } else {
                Color::DarkGray
            };
            // Show content with egg yolk tint during animation
            let style = if progress < 0.5 {
                Style::default().fg(egg_yolk)
            } else {
                content_style
            };
            (style, border_color, None)
        } else {
            (content_style, Color::DarkGray, None)
        }
    } else if is_flashing && flash_progress < 1.0 {
        // Flashing cell: bright egg yolk border that fades
        let intensity = 1.0 - flash_progress;
        let border_color = if intensity > 0.5 {
            egg_yolk
        } else {
            Color::Yellow
        };
        (content_style, border_color, None)
    } else if is_cursor {
        // Cursor cell: bright egg yolk border
        (content_style, egg_yolk, None)
    } else {
        (content_style, Color::DarkGray, None)
    };

    // Create the border block (thicker for cursor, rounded for others)
    let border_type = if is_cursor {
        BorderType::Thick
    } else {
        BorderType::Rounded
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(border_color));

    // Get inner area (inside borders)
    let inner_area = block.inner(area);

    // Render the border first
    frame.render_widget(block, area);

    // If there's an inner background, fill it
    if let Some(bg_color) = inner_bg {
        let bg = Block::default().style(Style::default().bg(bg_color));
        frame.render_widget(bg, inner_area);
    }

    // Render the content on top
    let cell_content = Paragraph::new(content)
        .alignment(Alignment::Center)
        .style(cell_style);

    frame.render_widget(cell_content, inner_area);
}

/// Get the content character and style for a cell.
fn get_cell_content(board: &Board, state: &GameState, row: usize, col: usize) -> (String, Style) {
    let is_revealed = board.is_revealed(row, col);
    let is_egg = board.is_egg(row, col);
    let is_collected = state.eggs_collected.contains(&(row, col));

    if !is_revealed {
        // Unrevealed cells show blank (background color indicates hidden state)
        (" ".to_string(), Style::default())
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

/// Render the game over screen with a popup overlay on the game board.
pub fn render_game_over(frame: &mut Frame, board: &Board, state: &GameState) {
    // First render the game board behind the popup (no animations)
    render_game(frame, board, state, None, &[]);

    let eggs = state.eggs_collected.len();
    let total = board.egg_count;

    let (message, message_color) = if eggs == total {
        ("🎉 PERFECT! 🎉", Color::Green)
    } else if eggs == 0 {
        ("💸 Inflation won 📈", Color::Red)
    } else {
        ("Game Over!", Color::Yellow)
    };

    let savings = eggs * 7;

    let lines = vec![
        Line::from(Span::styled(
            message,
            Style::default()
                .fg(message_color)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("Eggs: {} / {}", eggs, total)),
        Line::from(format!("Saved: ${}", savings)),
        Line::from(""),
        Line::from(Span::styled(
            "R: restart | Q: quit",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    // Calculate the board area (same layout as render_game)
    let area = frame.area();
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);
    let game_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(22)])
        .split(main_chunks[1]);
    let board_area = game_chunks[0];

    // Calculate popup size and center on board area
    let popup_width = 24;
    let popup_height = 8;
    let popup_area = center_rect(board_area, popup_width, popup_height);

    // Clear the popup area first (removes underlying content)
    frame.render_widget(Clear, popup_area);

    // Render solid black background
    let bg = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(bg, popup_area);

    // Render popup content
    let popup = Paragraph::new(lines)
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(EGG_BORDER_STYLE)
                .border_type(BorderType::Rounded)
                .title("Game Over"),
        )
        .alignment(Alignment::Center);

    frame.render_widget(popup, popup_area);
}
