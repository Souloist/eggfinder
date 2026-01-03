//! UI rendering using ratatui.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::board::Board;
use crate::constants::{CellDisplay, CellType, Difficulty};
use crate::game_state::GameState;

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
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED)
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
        .block(Block::default().borders(Borders::ALL).title("EggFinder"))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(menu, area);
}

/// Render the game board and status bar.
pub fn render_game(frame: &mut Frame, board: &Board, state: &GameState) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    render_status(frame, chunks[0], board, state);
    render_board(frame, chunks[1], board, state);
}

fn render_status(frame: &mut Frame, area: Rect, board: &Board, state: &GameState) {
    let status_text = format!(
        "Turns: {} | Eggs: {}/{} | Score: {} | WASD: move | SPACE: reveal | Q: quit",
        state.turns_remaining,
        state.eggs_collected.len(),
        board.egg_count,
        state.score
    );

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Status"));

    frame.render_widget(status, area);
}

fn render_board(frame: &mut Frame, area: Rect, board: &Board, state: &GameState) {
    let mut lines: Vec<Line> = Vec::new();

    // Column headers
    let mut header_spans = vec![Span::raw("   ")];
    for col in 0..board.width {
        header_spans.push(Span::styled(
            format!("{:2}", col % 10),
            Style::default().fg(Color::DarkGray),
        ));
    }
    lines.push(Line::from(header_spans));

    for row in 0..board.height {
        let mut row_spans = vec![Span::styled(
            format!("{:2} ", row % 100),
            Style::default().fg(Color::DarkGray),
        )];

        for col in 0..board.width {
            let (ch, style) = get_cell_display(board, state, row, col);
            row_spans.push(Span::styled(format!("{} ", ch), style));
        }

        lines.push(Line::from(row_spans));
    }

    let board_widget =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Board"));

    frame.render_widget(board_widget, area);
}

/// Determine the display character and style for a cell.
fn get_cell_display(board: &Board, state: &GameState, row: usize, col: usize) -> (char, Style) {
    let is_cursor = state.is_cursor_at(row, col);
    let is_revealed = board.is_revealed(row, col);
    let is_egg = board.is_egg(row, col);
    let is_collected = state.eggs_collected.contains(&(row, col));

    let base_style = if is_cursor {
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    if !is_revealed {
        (CellDisplay::HIDDEN, base_style.fg(Color::Blue))
    } else if is_egg {
        if is_collected {
            (CellDisplay::COLLECTED_EGG, base_style.fg(Color::Green))
        } else {
            // Revealed but not collected (shown at game end)
            (CellDisplay::REVEALED_EGG, base_style.fg(Color::Yellow))
        }
    } else {
        let cell_value = board.cells[row][col];
        if cell_value == CellType::Empty.value() {
            (CellDisplay::EMPTY, base_style.fg(Color::DarkGray))
        } else {
            // Number = adjacent egg count
            let ch = char::from_digit(cell_value as u32, 10).unwrap_or('?');
            let color = match cell_value {
                1 => Color::Blue,
                2 => Color::Green,
                3 => Color::Red,
                4 => Color::Magenta,
                5 => Color::Yellow,
                _ => Color::White,
            };
            (ch, base_style.fg(color))
        }
    }
}

/// Render the game over screen.
pub fn render_game_over(frame: &mut Frame, board: &Board, state: &GameState) {
    let area = frame.area();

    let message = if state.eggs_collected.len() == board.egg_count {
        "🎉 Perfect! You collected all the eggs! 🎉"
    } else {
        "Game Over!"
    };

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("Final Score: {}/{}", state.score, board.egg_count)),
        Line::from(format!(
            "Eggs Collected: {}/{}",
            state.eggs_collected.len(),
            board.egg_count
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press Q to quit or R to restart",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let game_over = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Game Over"))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(game_over, area);
}
