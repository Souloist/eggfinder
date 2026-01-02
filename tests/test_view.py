"""Unit tests for view.py (rendering functions)."""

import unittest
from eggfinder.board import Board
from eggfinder.model import GameState
from eggfinder.view import render_board, render_game_status, render_full_game


class TestRenderBoard(unittest.TestCase):
    """Test board rendering."""

    def test_render_board_returns_string(self):
        board = Board(3, 3, 1)
        state = GameState()
        result = render_board(board, state)
        self.assertIsInstance(result, str)

    def test_render_board_has_column_headers(self):
        board = Board(5, 5, 2)
        state = GameState()
        result = render_board(board, state)

        # Should contain column numbers
        lines = result.split('\n')
        self.assertGreater(len(lines), 0)
        # First line should have column headers
        for col in range(5):
            self.assertIn(str(col), lines[0])

    def test_render_board_hidden_cells(self):
        board = Board(3, 3, 1)
        state = GameState()
        result = render_board(board, state)

        # All cells should be hidden initially
        self.assertIn('#', result)

    def test_render_board_revealed_empty(self):
        board = Board(3, 3, 0)  # No eggs - all zeros
        state = GameState()
        board.revealed[1][1] = True

        result = render_board(board, state)
        self.assertIn('.', result)

    def test_render_board_collected_egg(self):
        board = Board(3, 3, 0)
        board.eggs.add((1, 1))
        board.cells[1][1] = -1
        board.revealed[1][1] = True

        state = GameState()
        state.eggs_collected.add((1, 1))

        result = render_board(board, state)
        self.assertIn('*', result)

    def test_render_board_revealed_egg_not_collected(self):
        board = Board(3, 3, 0)
        board.eggs.add((1, 1))
        board.cells[1][1] = -1
        board.revealed[1][1] = True

        state = GameState()
        # Don't add to eggs_collected

        result = render_board(board, state)
        self.assertIn('E', result)
        self.assertNotIn('*', result)

    def test_render_board_show_all(self):
        board = Board(3, 3, 1)
        state = GameState()

        result = render_board(board, state, show_all=True)

        # Should not have hidden cells when show_all=True
        # At least numbers or dots should be visible
        self.assertTrue('E' in result or '.' in result or any(str(i) in result for i in range(1, 9)))

    def test_render_board_numbers(self):
        board = Board(3, 3, 0)
        board.eggs.add((1, 1))
        board.cells[1][1] = -1
        board._calculate_numbers()

        # Reveal a cell with a number
        board.revealed[0][0] = True

        state = GameState()
        result = render_board(board, state)

        # Should show '1' for cells adjacent to the egg
        self.assertIn('1', result)


class TestRenderGameStatus(unittest.TestCase):
    """Test game status rendering."""

    def test_render_status_returns_string(self):
        board = Board(5, 5, 3)
        state = GameState()
        result = render_game_status(state, board)
        self.assertIsInstance(result, str)

    def test_render_status_shows_turns(self):
        board = Board(5, 5, 3)
        state = GameState(turns_remaining=7)
        result = render_game_status(state, board)

        self.assertIn('7', result)
        self.assertIn('Turns', result)

    def test_render_status_shows_eggs_collected(self):
        board = Board(5, 5, 5)
        state = GameState()
        state.eggs_collected.add((0, 0))
        state.eggs_collected.add((1, 1))

        result = render_game_status(state, board)

        self.assertIn('2', result)  # Eggs collected
        self.assertIn('5', result)  # Total eggs
        self.assertIn('Eggs', result)

    def test_render_status_shows_score(self):
        board = Board(5, 5, 3)
        state = GameState(score=10)
        result = render_game_status(state, board)

        self.assertIn('10', result)
        self.assertIn('Score', result)

    def test_render_status_game_over(self):
        board = Board(5, 5, 3)
        state = GameState(game_over=True)
        result = render_game_status(state, board)

        self.assertIn('GAME OVER', result)

    def test_render_status_perfect_game(self):
        board = Board(5, 5, 3)
        state = GameState(game_over=True)
        # Collect all eggs
        state.eggs_collected = {(0, 0), (1, 1), (2, 2)}

        result = render_game_status(state, board)

        self.assertIn('GAME OVER', result)
        self.assertIn('all', result.lower())


class TestRenderFullGame(unittest.TestCase):
    """Test full game rendering."""

    def test_render_full_game_returns_string(self):
        board = Board(5, 5, 3)
        state = GameState()
        result = render_full_game(board, state)
        self.assertIsInstance(result, str)

    def test_render_full_game_contains_status_and_board(self):
        board = Board(5, 5, 3)
        state = GameState(turns_remaining=7, score=2)

        result = render_full_game(board, state)

        # Should contain elements from both status and board
        self.assertIn('Turns', result)
        self.assertIn('Score', result)
        self.assertIn('#', result)  # Hidden cells from board

    def test_render_full_game_multiline(self):
        board = Board(3, 3, 1)
        state = GameState()
        result = render_full_game(board, state)

        lines = result.split('\n')
        self.assertGreater(len(lines), 5)  # Status + board should have multiple lines

    def test_render_full_game_show_all(self):
        board = Board(3, 3, 1)
        state = GameState()

        result_hidden = render_full_game(board, state, show_all=False)
        result_shown = render_full_game(board, state, show_all=True)

        # When show_all=False, should have hidden cells
        self.assertIn('#', result_hidden)

        # Results should be different
        self.assertNotEqual(result_hidden, result_shown)


if __name__ == '__main__':
    unittest.main()
