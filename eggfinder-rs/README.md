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

1. **Select difficulty** - Choose Easy, Medium, or Hard
2. **Navigate** - Move the cursor around the board
3. **Reveal cells** - Press Space to reveal the cell under the cursor
4. **Collect eggs** - Find all the eggs before you run out of turns!
5. **Survive** - Game ends when you run out of turns or collect all eggs

### Difficulty Levels

| Level  | Board Size | Eggs | Starting Turns |
|--------|------------|------|----------------|
| Easy   | 8×8        | 3    | 4              |
| Medium | 15×12      | 7    | 7              |
| Hard   | 20×15      | 15   | 10             |

## Controls

| Key | Action |
|-----|--------|
| `W` / `↑` | Move up |
| `S` / `↓` | Move down |
| `A` / `←` | Move left |
| `D` / `→` | Move right |
| `Space` / `Enter` | Reveal cell / Select menu item |
| `B` | Back to difficulty selection |
| `R` | Restart (on game over screen) |
| `Q` / `Esc` | Quit |

## Game Rules

- Starting turns depend on difficulty level
- Clicking a **normal cell** costs 1 turn
- Clicking an **egg** also costs 1 turn (no refund!)
- Numbers show how many eggs are adjacent to that cell
- Goal: Collect all eggs before running out of turns!

## Board Symbols

| Symbol | Meaning |
|--------|---------|
| Brown tile | Hidden cell |
| (blank) | Empty (no adjacent eggs) |
| `1-8` | Number of adjacent eggs |
| 🥚 | Collected egg |
| 💩 | Missed egg (shown at game end) |

## Requirements

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- A terminal that supports Unicode and emoji

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
