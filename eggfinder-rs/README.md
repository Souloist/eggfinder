# EggFinder TUI

A terminal-based minesweeper-like game where you **collect eggs** instead of avoiding mines. Built with Rust and [ratatui](https://ratatui.rs/).

## Quick Start

```bash
# Build and run
cargo run

# Or build release version (faster)
cargo run --release
```

## How to Play

1. **Select difficulty** — Choose Easy, Medium, or Hard
2. **Navigate** — Move the cursor around the board
3. **Reveal cells** — Press Space to reveal the cell under the cursor
4. **Collect eggs** — Finding an egg gives you a bonus turn!
5. **Survive** — Game ends when you run out of turns

### Difficulty Levels

| Level  | Board Size | Eggs |
|--------|------------|------|
| Easy   | 9×9        | 6    |
| Medium | 16×16      | 8    |
| Hard   | 25×25      | 10   |

## Controls

| Key | Action |
|-----|--------|
| `W` / `↑` | Move up |
| `S` / `↓` | Move down |
| `A` / `←` | Move left |
| `D` / `→` | Move right |
| `Space` / `Enter` | Reveal cell / Select menu item |
| `R` | Restart (on game over screen) |
| `Q` / `Esc` | Quit |

## Game Rules

- You start with **10 turns**
- Clicking a **normal cell** costs 1 turn
- Clicking an **egg** costs 0 turns (you get +1 bonus turn)
- Numbers show how many eggs are adjacent to that cell
- Goal: Collect as many eggs as possible before running out of turns!

## Board Symbols

| Symbol | Meaning |
|--------|---------|
| `■` | Hidden cell |
| `·` | Empty (no adjacent eggs) |
| `1-8` | Number of adjacent eggs |
| `★` | Collected egg |
| `○` | Revealed egg (shown at game end) |

## Requirements

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- A terminal that supports Unicode and 256 colors

## Development

```bash
# Check for errors
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint with clippy
cargo clippy
```
