from collections import deque

from .board import Board
from .constants import CellType, GameConfig
from .model import GameState
from .types import ClickResult


def floodfill_reveal(board: Board, row: int, col: int) -> int:
    """BFS floodfill to reveal connected empty cells and border numbers.

    Eggs are never revealed by floodfill (matching minesweeper behavior).
    """
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

        if board.is_egg(r, c):
            continue

        visited.add((r, c))
        board.revealed[r][c] = True
        cells_revealed += 1

        cell_value = board.cells[r][c]

        if cell_value == CellType.EMPTY:
            for nr, nc in board.get_neighbors(r, c):
                if (nr, nc) not in visited:
                    queue.append((nr, nc))

    return cells_revealed


def _validate_click(board: Board, game_state: GameState, row: int, col: int) -> ClickResult | None:
    """Validate click preconditions. Returns ClickResult if invalid, None if valid."""
    if game_state.game_over:
        return ClickResult.invalid("Game is already over!", game_over=True)

    if not board.is_valid_position(row, col):
        return ClickResult.invalid(
            f"Invalid position ({row}, {col}). Must be within bounds: 0-{board.height - 1}, 0-{board.width - 1}"
        )

    if board.revealed[row][col]:
        return ClickResult.invalid(f"Cell ({row}, {col}) is already revealed!")

    return None


def _handle_egg_click(board: Board, game_state: GameState, row: int, col: int) -> ClickResult:
    """Handle clicking on an egg cell."""
    game_state.eggs_collected.add((row, col))
    board.revealed[row][col] = True
    game_state.turns_remaining += GameConfig.EGG_BONUS_TURNS
    game_state.score = calculate_score(game_state)

    game_over = game_state.turns_remaining <= 0
    if game_over:
        game_state.game_over = True

    message = f"Found an egg at ({row}, {col})! +{GameConfig.EGG_BONUS_TURNS} turns. Eggs collected: {len(game_state.eggs_collected)}/{board.egg_count}"

    return ClickResult.egg_collected(
        message=message, turns_delta=GameConfig.EGG_BONUS_TURNS, game_over=game_over
    )


def _handle_cell_click(board: Board, game_state: GameState, row: int, col: int) -> ClickResult:
    """Handle clicking on a non-egg cell."""
    cells_revealed = floodfill_reveal(board, row, col)
    game_state.turns_remaining -= 1

    game_over = game_state.turns_remaining <= 0
    if game_over:
        game_state.game_over = True

    cell_value = board.cells[row][col]
    if cell_value == CellType.EMPTY:
        message = f"Revealed {cells_revealed} cells. Turns remaining: {game_state.turns_remaining}"
    else:
        message = f"Revealed number {cell_value}. Turns remaining: {game_state.turns_remaining}"

    return ClickResult.cell_revealed(
        message=message, cells_revealed=cells_revealed, game_over=game_over
    )


def process_click(board: Board, game_state: GameState, row: int, col: int) -> ClickResult:
    """Process a player's click. Mutates board and game_state."""
    validation_result = _validate_click(board, game_state, row, col)
    if validation_result is not None:
        return validation_result

    if board.is_egg(row, col):
        return _handle_egg_click(board, game_state, row, col)
    else:
        return _handle_cell_click(board, game_state, row, col)


def check_game_over(game_state: GameState) -> bool:
    """Check if game should end."""
    return game_state.turns_remaining <= 0


def calculate_score(game_state: GameState) -> int:
    """Calculate current score based on eggs collected."""
    return len(game_state.eggs_collected)
