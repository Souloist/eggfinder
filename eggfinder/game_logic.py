from collections import deque
from typing import Dict, Any
from .board import Board
from .model import GameState


def floodfill_reveal(board: Board, row: int, col: int) -> int:
    """BFS floodfill to reveal connected empty cells and border numbers."""
    queue = deque([(row, col)])
    visited = set()
    cells_revealed = 0

    while queue:
        r, c = queue.popleft()

        if (r, c) in visited:
            continue

        if not board.is_valid_position(r, c):
            continue

        if board.revealed[r][c]:
            continue

        visited.add((r, c))
        board.revealed[r][c] = True
        cells_revealed += 1

        cell_value = board.cells[r][c]

        if cell_value == 0:
            for nr, nc in board.get_neighbors(r, c):
                if (nr, nc) not in visited:
                    queue.append((nr, nc))

    return cells_revealed


def process_click(board: Board, game_state: GameState, row: int, col: int) -> Dict[str, Any]:
    """Process a player's click. Mutates board and game_state."""
    if game_state.game_over:
        return {
            'valid': False,
            'message': 'Game is already over!',
            'egg_found': False,
            'turns_used': 0,
            'cells_revealed': 0,
            'game_over': True
        }

    if not board.is_valid_position(row, col):
        return {
            'valid': False,
            'message': f'Invalid position ({row}, {col}). Must be within bounds: 0-{board.height-1}, 0-{board.width-1}',
            'egg_found': False,
            'turns_used': 0,
            'cells_revealed': 0,
            'game_over': game_state.game_over
        }

    if board.revealed[row][col]:
        return {
            'valid': False,
            'message': f'Cell ({row}, {col}) is already revealed!',
            'egg_found': False,
            'turns_used': 0,
            'cells_revealed': 0,
            'game_over': game_state.game_over
        }

    if board.is_egg(row, col):
        game_state.eggs_collected.add((row, col))
        board.revealed[row][col] = True
        game_state.turns_remaining += 2
        game_state.score = calculate_score(game_state)

        if game_state.turns_remaining <= 0:
            game_state.game_over = True

        return {
            'valid': True,
            'message': f'Found an egg at ({row}, {col})! +2 turns. Eggs collected: {len(game_state.eggs_collected)}/{board.egg_count}',
            'egg_found': True,
            'turns_used': -2,
            'cells_revealed': 1,
            'game_over': game_state.game_over
        }

    cells_revealed = floodfill_reveal(board, row, col)
    game_state.turns_remaining -= 1

    if game_state.turns_remaining <= 0:
        game_state.game_over = True

    cell_value = board.cells[row][col]
    if cell_value == 0:
        message = f'Revealed {cells_revealed} cells. Turns remaining: {game_state.turns_remaining}'
    else:
        message = f'Revealed number {cell_value}. Turns remaining: {game_state.turns_remaining}'

    return {
        'valid': True,
        'message': message,
        'egg_found': False,
        'turns_used': 1,
        'cells_revealed': cells_revealed,
        'game_over': game_state.game_over
    }


def check_game_over(game_state: GameState) -> bool:
    return game_state.turns_remaining <= 0


def calculate_score(game_state: GameState) -> int:
    return len(game_state.eggs_collected)
