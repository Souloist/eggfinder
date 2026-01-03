"""EggFinder - A minesweeper-like game where you collect eggs instead of avoiding mines.

This package follows Elm/MVC architecture for easy porting to Rust + TUI:
- model: Pure data structures (GameState)
- board: Board operations and state
- game_logic: Game rules and update functions
- view: Rendering functions

Architecture:
    State + Action → New State → Render
"""

from .board import Board
from .game_logic import calculate_score, check_game_over, floodfill_reveal, process_click
from .model import GameState
from .view import render_board, render_full_game, render_game_status

__all__ = [
    "Board",
    # Core types
    "GameState",
    "calculate_score",
    "check_game_over",
    "floodfill_reveal",
    # Game logic
    "process_click",
    "render_board",
    # Display
    "render_full_game",
    "render_game_status",
]

__version__ = "0.1.0"
