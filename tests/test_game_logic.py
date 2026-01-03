"""Unit tests for game_logic.py."""

import unittest

from eggfinder.board import Board
from eggfinder.game_logic import calculate_score, check_game_over, floodfill_reveal, process_click
from eggfinder.model import GameState


class TestCalculateScore(unittest.TestCase):
    def test_score_no_eggs(self):
        state = GameState()
        self.assertEqual(calculate_score(state), 0)

    def test_score_with_eggs(self):
        state = GameState()
        state.eggs_collected.add((0, 0))
        self.assertEqual(calculate_score(state), 1)

        state.eggs_collected.add((1, 1))
        self.assertEqual(calculate_score(state), 2)

        state.eggs_collected.add((2, 2))
        self.assertEqual(calculate_score(state), 3)


class TestCheckGameOver(unittest.TestCase):
    def test_game_not_over(self):
        state = GameState(turns_remaining=5)
        self.assertFalse(check_game_over(state))

    def test_game_over_zero_turns(self):
        state = GameState(turns_remaining=0)
        self.assertTrue(check_game_over(state))

    def test_game_over_negative_turns(self):
        state = GameState(turns_remaining=-1)
        self.assertTrue(check_game_over(state))


class TestFloodfillReveal(unittest.TestCase):
    def test_floodfill_single_cell(self):
        board = Board(3, 3, 0)
        board.cells[1][1] = -1
        board.eggs.add((1, 1))
        board._calculate_numbers()

        cells_revealed = floodfill_reveal(board, 0, 0)
        self.assertEqual(cells_revealed, 1)
        self.assertTrue(board.revealed[0][0])

    def test_floodfill_empty_region(self):
        board = Board(5, 5, 0)

        cells_revealed = floodfill_reveal(board, 0, 0)
        self.assertEqual(cells_revealed, 25)

        for row in board.revealed:
            for cell in row:
                self.assertTrue(cell)

    def test_floodfill_stops_at_numbers(self):
        board = Board(5, 5, 0)

        board.eggs.add((0, 0))
        board.cells[0][0] = -1
        board._calculate_numbers()

        cells_revealed = floodfill_reveal(board, 4, 4)

        self.assertGreater(cells_revealed, 1)
        self.assertTrue(board.revealed[4][4])

    def test_floodfill_already_revealed(self):
        board = Board(3, 3, 0)
        board.revealed[1][1] = True

        cells_revealed = floodfill_reveal(board, 1, 1)
        self.assertEqual(cells_revealed, 0)

    def test_floodfill_does_not_reveal_eggs(self):
        board = Board(5, 5, 0)

        board.eggs.add((2, 2))
        board.cells[2][2] = -1
        board._calculate_numbers()

        floodfill_reveal(board, 0, 0)

        self.assertFalse(board.revealed[2][2])


class TestProcessClick(unittest.TestCase):
    def test_click_out_of_bounds(self):
        board = Board(5, 5, 3)
        state = GameState()

        result = process_click(board, state, -1, 0)
        self.assertFalse(result.valid)
        self.assertEqual(state.turns_remaining, 10)

        result = process_click(board, state, 5, 5)
        self.assertFalse(result.valid)
        self.assertEqual(state.turns_remaining, 10)

    def test_click_already_revealed(self):
        board = Board(5, 5, 3)
        state = GameState()

        board.revealed[2][2] = True
        result = process_click(board, state, 2, 2)

        self.assertFalse(result.valid)
        self.assertEqual(state.turns_remaining, 10)

    def test_click_game_already_over(self):
        board = Board(5, 5, 3)
        state = GameState(game_over=True)

        result = process_click(board, state, 0, 0)
        self.assertFalse(result.valid)
        self.assertIn("already over", result.message.lower())

    def test_click_on_egg(self):
        board = Board(5, 5, 0)
        board.eggs.add((2, 2))
        board.cells[2][2] = -1
        board._calculate_numbers()

        state = GameState()
        initial_turns = state.turns_remaining

        result = process_click(board, state, 2, 2)

        self.assertTrue(result.valid)
        self.assertTrue(result.egg_found)
        self.assertIn((2, 2), state.eggs_collected)
        self.assertEqual(state.turns_remaining, initial_turns + 1)
        self.assertEqual(state.score, 1)
        self.assertTrue(board.revealed[2][2])

    def test_click_on_number(self):
        board = Board(5, 5, 0)
        board.eggs.add((0, 0))
        board.cells[0][0] = -1
        board._calculate_numbers()

        state = GameState()
        initial_turns = state.turns_remaining

        result = process_click(board, state, 0, 1)

        self.assertTrue(result.valid)
        self.assertFalse(result.egg_found)
        self.assertEqual(state.turns_remaining, initial_turns - 1)
        self.assertTrue(board.revealed[0][1])

    def test_click_on_empty_triggers_floodfill(self):
        board = Board(5, 5, 0)
        state = GameState()

        result = process_click(board, state, 0, 0)

        self.assertTrue(result.valid)
        self.assertGreater(result.cells_revealed, 1)
        self.assertEqual(state.turns_remaining, 9)

    def test_game_over_on_last_turn(self):
        board = Board(5, 5, 0)
        state = GameState(turns_remaining=1)

        result = process_click(board, state, 0, 0)

        self.assertTrue(result.valid)
        self.assertTrue(state.game_over)
        self.assertEqual(state.turns_remaining, 0)

    def test_multiple_egg_collection(self):
        board = Board(5, 5, 0)
        board.eggs.add((0, 0))
        board.eggs.add((1, 1))
        board.eggs.add((2, 2))
        board.cells[0][0] = -1
        board.cells[1][1] = -1
        board.cells[2][2] = -1
        board._calculate_numbers()

        state = GameState()

        process_click(board, state, 0, 0)
        self.assertEqual(state.turns_remaining, 11)
        self.assertEqual(state.score, 1)

        process_click(board, state, 1, 1)
        self.assertEqual(state.turns_remaining, 12)
        self.assertEqual(state.score, 2)

        process_click(board, state, 2, 2)
        self.assertEqual(state.turns_remaining, 13)
        self.assertEqual(state.score, 3)


class TestProcessClickEdgeCases(unittest.TestCase):
    def test_click_returns_correct_structure(self):
        board = Board(3, 3, 1)
        state = GameState()

        result = process_click(board, state, 0, 0)

        self.assertTrue(hasattr(result, "valid"))
        self.assertTrue(hasattr(result, "message"))
        self.assertTrue(hasattr(result, "egg_found"))
        self.assertTrue(hasattr(result, "turns_delta"))
        self.assertTrue(hasattr(result, "cells_revealed"))
        self.assertTrue(hasattr(result, "game_over"))

    def test_revealed_egg_not_collected(self):
        """Eggs should never be revealed by floodfill, only by direct clicks."""
        board = Board(5, 5, 0)

        board.eggs.add((2, 2))
        board.cells[2][2] = -1
        board._calculate_numbers()

        state = GameState()

        process_click(board, state, 0, 0)

        self.assertFalse(board.revealed[2][2])
        self.assertNotIn((2, 2), state.eggs_collected)


if __name__ == "__main__":
    unittest.main()
