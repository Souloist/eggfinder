"""Unit tests for model.py (GameState)."""

import unittest

from eggfinder.model import GameState


class TestGameState(unittest.TestCase):
    def test_default_initialization(self):
        state = GameState()
        self.assertEqual(state.turns_remaining, 10)
        self.assertEqual(len(state.eggs_collected), 0)
        self.assertFalse(state.game_over)
        self.assertEqual(state.score, 0)

    def test_custom_initialization(self):
        state = GameState(turns_remaining=5, score=3, game_over=True)
        self.assertEqual(state.turns_remaining, 5)
        self.assertEqual(state.score, 3)
        self.assertTrue(state.game_over)

    def test_eggs_collected_mutation(self):
        state = GameState()
        state.eggs_collected.add((0, 0))
        state.eggs_collected.add((1, 1))
        self.assertEqual(len(state.eggs_collected), 2)
        self.assertIn((0, 0), state.eggs_collected)
        self.assertIn((1, 1), state.eggs_collected)

    def test_turns_modification(self):
        state = GameState()
        state.turns_remaining -= 1
        self.assertEqual(state.turns_remaining, 9)

        state.turns_remaining += 2
        self.assertEqual(state.turns_remaining, 11)

    def test_game_over_flag(self):
        state = GameState()
        self.assertFalse(state.game_over)

        state.game_over = True
        self.assertTrue(state.game_over)

    def test_score_update(self):
        state = GameState()
        state.score = 5
        self.assertEqual(state.score, 5)


if __name__ == "__main__":
    unittest.main()
