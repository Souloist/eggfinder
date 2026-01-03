"""Unit tests for view.py (rendering functions)."""

from eggfinder.board import Board
from eggfinder.model import GameState
from eggfinder.view import render_board, render_full_game, render_game_status


class TestRenderBoard:
    """Test board rendering."""

    def test_render_board_returns_string(self) -> None:
        board = Board(3, 3, 1)
        state = GameState()
        result = render_board(board, state)
        assert isinstance(result, str)

    def test_render_board_has_column_headers(self) -> None:
        board = Board(5, 5, 2)
        state = GameState()
        result = render_board(board, state)

        # Should contain column numbers
        lines = result.split("\n")
        assert len(lines) > 0
        # First line should have column headers
        for col in range(5):
            assert str(col) in lines[0]

    def test_render_board_hidden_cells(self) -> None:
        board = Board(3, 3, 1)
        state = GameState()
        result = render_board(board, state)

        # All cells should be hidden initially
        assert "■" in result

    def test_render_board_revealed_empty(self) -> None:
        board = Board(3, 3, 0)  # No eggs - all zeros
        state = GameState()
        board.revealed[1][1] = True

        result = render_board(board, state)
        assert "·" in result

    def test_render_board_collected_egg(self) -> None:
        board = Board(3, 3, 0)
        board.eggs.add((1, 1))
        board.cells[1][1] = -1
        board.revealed[1][1] = True

        state = GameState()
        state.eggs_collected.add((1, 1))

        result = render_board(board, state)
        assert "★" in result

    def test_render_board_revealed_egg_not_collected(self) -> None:
        board = Board(3, 3, 0)
        board.eggs.add((1, 1))
        board.cells[1][1] = -1
        board.revealed[1][1] = True

        state = GameState()
        # Don't add to eggs_collected

        result = render_board(board, state)
        assert "○" in result
        assert "★" not in result

    def test_render_board_show_all(self) -> None:
        board = Board(3, 3, 1)
        state = GameState()

        result = render_board(board, state, show_all=True)

        # Should not have hidden cells when show_all=True
        # At least numbers or dots should be visible
        assert "○" in result or "·" in result or any(str(i) in result for i in range(1, 9))

    def test_render_board_numbers(self) -> None:
        board = Board(3, 3, 0)
        board.eggs.add((1, 1))
        board.cells[1][1] = -1
        board._calculate_numbers()

        # Reveal a cell with a number
        board.revealed[0][0] = True

        state = GameState()
        result = render_board(board, state)

        # Should show '1' for cells adjacent to the egg
        assert "1" in result


class TestRenderGameStatus:
    """Test game status rendering."""

    def test_render_status_returns_string(self) -> None:
        board = Board(5, 5, 3)
        state = GameState()
        result = render_game_status(state, board)
        assert isinstance(result, str)

    def test_render_status_shows_turns(self) -> None:
        board = Board(5, 5, 3)
        state = GameState(turns_remaining=7)
        result = render_game_status(state, board)

        assert "7" in result
        assert "Turns" in result

    def test_render_status_shows_eggs_collected(self) -> None:
        board = Board(5, 5, 5)
        state = GameState()
        state.eggs_collected.add((0, 0))
        state.eggs_collected.add((1, 1))

        result = render_game_status(state, board)

        assert "2" in result  # Eggs collected
        assert "5" in result  # Total eggs
        assert "Eggs" in result

    def test_render_status_shows_score(self) -> None:
        board = Board(5, 5, 3)
        state = GameState(score=10)
        result = render_game_status(state, board)

        assert "10" in result
        assert "Score" in result

    def test_render_status_game_over(self) -> None:
        board = Board(5, 5, 3)
        state = GameState(game_over=True)
        result = render_game_status(state, board)

        assert "GAME OVER" in result

    def test_render_status_perfect_game(self) -> None:
        board = Board(5, 5, 3)
        state = GameState(game_over=True)
        # Collect all eggs
        state.eggs_collected = {(0, 0), (1, 1), (2, 2)}

        result = render_game_status(state, board)

        assert "GAME OVER" in result
        assert "all" in result.lower()


class TestRenderFullGame:
    """Test full game rendering."""

    def test_render_full_game_returns_string(self) -> None:
        board = Board(5, 5, 3)
        state = GameState()
        result = render_full_game(board, state)
        assert isinstance(result, str)

    def test_render_full_game_contains_status_and_board(self) -> None:
        board = Board(5, 5, 3)
        state = GameState(turns_remaining=7, score=2)

        result = render_full_game(board, state)

        # Should contain elements from both status and board
        assert "Turns" in result
        assert "Score" in result
        assert "■" in result  # Hidden cells from board

    def test_render_full_game_multiline(self) -> None:
        board = Board(3, 3, 1)
        state = GameState()
        result = render_full_game(board, state)

        lines = result.split("\n")
        assert len(lines) > 5  # Status + board should have multiple lines

    def test_render_full_game_show_all(self) -> None:
        board = Board(3, 3, 1)
        state = GameState()

        result_hidden = render_full_game(board, state, show_all=False)
        result_shown = render_full_game(board, state, show_all=True)

        # When show_all=False, should have hidden cells
        assert "■" in result_hidden

        # Results should be different
        assert result_hidden != result_shown
