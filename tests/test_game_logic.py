"""Unit tests for game_logic.py."""

from eggfinder.board import Board
from eggfinder.game_logic import calculate_score, check_game_over, floodfill_reveal, process_click
from eggfinder.model import GameState


class TestCalculateScore:
    def test_score_no_eggs(self) -> None:
        state = GameState()
        assert calculate_score(state) == 0

    def test_score_with_eggs(self) -> None:
        state = GameState()
        state.eggs_collected.add((0, 0))
        assert calculate_score(state) == 1

        state.eggs_collected.add((1, 1))
        assert calculate_score(state) == 2

        state.eggs_collected.add((2, 2))
        assert calculate_score(state) == 3


class TestCheckGameOver:
    def test_game_not_over(self) -> None:
        state = GameState(turns_remaining=5)
        assert not check_game_over(state)

    def test_game_over_zero_turns(self) -> None:
        state = GameState(turns_remaining=0)
        assert check_game_over(state)

    def test_game_over_negative_turns(self) -> None:
        state = GameState(turns_remaining=-1)
        assert check_game_over(state)


class TestFloodfillReveal:
    def test_floodfill_single_cell(self) -> None:
        board = Board(3, 3, 0)
        board.cells[1][1] = -1
        board.eggs.add((1, 1))
        board._calculate_numbers()

        cells_revealed = floodfill_reveal(board, 0, 0)
        assert cells_revealed == 1
        assert board.revealed[0][0]

    def test_floodfill_empty_region(self) -> None:
        board = Board(5, 5, 0)

        cells_revealed = floodfill_reveal(board, 0, 0)
        assert cells_revealed == 25

        for row in board.revealed:
            for cell in row:
                assert cell

    def test_floodfill_stops_at_numbers(self) -> None:
        board = Board(5, 5, 0)

        board.eggs.add((0, 0))
        board.cells[0][0] = -1
        board._calculate_numbers()

        cells_revealed = floodfill_reveal(board, 4, 4)

        assert cells_revealed > 1
        assert board.revealed[4][4]

    def test_floodfill_already_revealed(self) -> None:
        board = Board(3, 3, 0)
        board.revealed[1][1] = True

        cells_revealed = floodfill_reveal(board, 1, 1)
        assert cells_revealed == 0

    def test_floodfill_does_not_reveal_eggs(self) -> None:
        board = Board(5, 5, 0)

        board.eggs.add((2, 2))
        board.cells[2][2] = -1
        board._calculate_numbers()

        floodfill_reveal(board, 0, 0)

        assert not board.revealed[2][2]


class TestProcessClick:
    def test_click_out_of_bounds(self) -> None:
        board = Board(5, 5, 3)
        state = GameState()

        result = process_click(board, state, -1, 0)
        assert not result.valid
        assert state.turns_remaining == 10

        result = process_click(board, state, 5, 5)
        assert not result.valid
        assert state.turns_remaining == 10

    def test_click_already_revealed(self) -> None:
        board = Board(5, 5, 3)
        state = GameState()

        board.revealed[2][2] = True
        result = process_click(board, state, 2, 2)

        assert not result.valid
        assert state.turns_remaining == 10

    def test_click_game_already_over(self) -> None:
        board = Board(5, 5, 3)
        state = GameState(game_over=True)

        result = process_click(board, state, 0, 0)
        assert not result.valid
        assert "already over" in result.message.lower()

    def test_click_on_egg(self) -> None:
        board = Board(5, 5, 0)
        board.eggs.add((2, 2))
        board.cells[2][2] = -1
        board._calculate_numbers()

        state = GameState()
        initial_turns = state.turns_remaining

        result = process_click(board, state, 2, 2)

        assert result.valid
        assert result.egg_found
        assert (2, 2) in state.eggs_collected
        assert state.turns_remaining == initial_turns + 1
        assert state.score == 1
        assert board.revealed[2][2]

    def test_click_on_number(self) -> None:
        board = Board(5, 5, 0)
        board.eggs.add((0, 0))
        board.cells[0][0] = -1
        board._calculate_numbers()

        state = GameState()
        initial_turns = state.turns_remaining

        result = process_click(board, state, 0, 1)

        assert result.valid
        assert not result.egg_found
        assert state.turns_remaining == initial_turns - 1
        assert board.revealed[0][1]

    def test_click_on_empty_triggers_floodfill(self) -> None:
        board = Board(5, 5, 0)
        state = GameState()

        result = process_click(board, state, 0, 0)

        assert result.valid
        assert result.cells_revealed > 1
        assert state.turns_remaining == 9

    def test_game_over_on_last_turn(self) -> None:
        board = Board(5, 5, 0)
        state = GameState(turns_remaining=1)

        result = process_click(board, state, 0, 0)

        assert result.valid
        assert state.game_over
        assert state.turns_remaining == 0

    def test_multiple_egg_collection(self) -> None:
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
        assert state.turns_remaining == 11
        assert state.score == 1

        process_click(board, state, 1, 1)
        assert state.turns_remaining == 12
        assert state.score == 2

        process_click(board, state, 2, 2)
        assert state.turns_remaining == 13
        assert state.score == 3


class TestProcessClickEdgeCases:
    def test_click_returns_correct_structure(self) -> None:
        board = Board(3, 3, 1)
        state = GameState()

        result = process_click(board, state, 0, 0)

        assert hasattr(result, "valid")
        assert hasattr(result, "message")
        assert hasattr(result, "egg_found")
        assert hasattr(result, "turns_delta")
        assert hasattr(result, "cells_revealed")
        assert hasattr(result, "game_over")

    def test_revealed_egg_not_collected(self) -> None:
        """Eggs should never be revealed by floodfill, only by direct clicks."""
        board = Board(5, 5, 0)

        board.eggs.add((2, 2))
        board.cells[2][2] = -1
        board._calculate_numbers()

        state = GameState()

        process_click(board, state, 0, 0)

        assert not board.revealed[2][2]
        assert (2, 2) not in state.eggs_collected
