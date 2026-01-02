"""EggFinder - A minesweeper-like game where you collect eggs instead of avoiding mines.

This package follows Elm/MVC architecture for easy porting to Rust + TUI:
- model: Pure data structures (GameState)
- board: Board operations and state
- game_logic: Game rules and update functions
- view: Rendering functions

Architecture:
    State + Action → New State → Render
"""

from .model import GameState
from .board import Board
from .game_logic import process_click, floodfill_reveal, calculate_score, check_game_over
from .view import render_full_game, render_board, render_game_status

__all__ = [
    # Core types
    'GameState',
    'Board',
    # Game logic
    'process_click',
    'floodfill_reveal',
    'calculate_score',
    'check_game_over',
    # Display
    'render_full_game',
    'render_board',
    'render_game_status',
]

__version__ = '0.1.0'
