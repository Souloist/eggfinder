"""Unit tests for model.py (GameState)."""

from eggfinder.model import GameState


class TestGameState:
    def test_default_initialization(self) -> None:
        state = GameState()
        assert state.turns_remaining == 10
        assert len(state.eggs_collected) == 0
        assert not state.game_over
        assert state.score == 0

    def test_custom_initialization(self) -> None:
        state = GameState(turns_remaining=5, score=3, game_over=True)
        assert state.turns_remaining == 5
        assert state.score == 3
        assert state.game_over

    def test_eggs_collected_mutation(self) -> None:
        state = GameState()
        state.eggs_collected.add((0, 0))
        state.eggs_collected.add((1, 1))
        assert len(state.eggs_collected) == 2
        assert (0, 0) in state.eggs_collected
        assert (1, 1) in state.eggs_collected

    def test_turns_modification(self) -> None:
        state = GameState()
        state.turns_remaining -= 1
        assert state.turns_remaining == 9

        state.turns_remaining += 2
        assert state.turns_remaining == 11

    def test_game_over_flag(self) -> None:
        state = GameState()
        assert not state.game_over

        state.game_over = True
        assert state.game_over

    def test_score_update(self) -> None:
        state = GameState()
        state.score = 5
        assert state.score == 5
