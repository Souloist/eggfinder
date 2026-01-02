from .board import Board
from .model import GameState


def render_board(board: Board, game_state: GameState, show_all: bool = False) -> str:
    """Render board. Display: * = collected, E = revealed egg, # = hidden, . = empty, 1-8 = numbers."""
    lines = []

    header = "  "
    for col in range(board.width):
        header += f"{col:2}"
    lines.append(header)

    for row in range(board.height):
        line = f"{row:2}"
        for col in range(board.width):
            if show_all or board.revealed[row][col]:
                if (row, col) in game_state.eggs_collected:
                    line += " *"
                elif board.cells[row][col] == -1:
                    line += " E"
                elif board.cells[row][col] == 0:
                    line += " ."
                else:
                    line += f" {board.cells[row][col]}"
            else:
                line += " #"
        lines.append(line)

    return "\n".join(lines)


def render_game_status(game_state: GameState, board: Board) -> str:
    """Render game status bar."""
    lines = []
    lines.append("=" * 40)
    lines.append(f"Turns Remaining: {game_state.turns_remaining}")
    lines.append(f"Eggs Collected: {len(game_state.eggs_collected)}/{board.egg_count}")
    lines.append(f"Score: {game_state.score}")

    if game_state.game_over:
        lines.append("")
        lines.append("*** GAME OVER ***")
        if len(game_state.eggs_collected) == board.egg_count:
            lines.append("Perfect! You collected all eggs!")
        else:
            lines.append(f"You collected {len(game_state.eggs_collected)} out of {board.egg_count} eggs.")

    lines.append("=" * 40)
    return "\n".join(lines)


def render_full_game(board: Board, game_state: GameState, show_all: bool = False) -> str:
    """Render complete game view (status + board)."""
    return render_game_status(game_state, board) + "\n" + render_board(board, game_state, show_all)
