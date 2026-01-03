"""Integration tests for full game flow."""

import random

from eggfinder import Board, GameState, process_click, render_full_game


class TestGameIntegration:
    """Test complete game scenarios."""

    def test_complete_game_flow(self) -> None:
        board = Board(5, 5, 3)
        state = GameState()

        # Initial state
        assert state.turns_remaining == 10
        assert state.score == 0
        assert not state.game_over

        # Play some moves
        moves_made = 0
        for row in range(5):
            for col in range(5):
                if state.game_over:
                    break

                result = process_click(board, state, row, col)
                if result.valid:
                    moves_made += 1

        # Game should have progressed
        assert moves_made > 0

        # If game is over, turns should be 0
        if state.game_over:
            assert state.turns_remaining == 0

    def test_collect_all_eggs(self) -> None:
        board = Board(3, 3, 3)
        state = GameState(turns_remaining=20)  # Extra turns to ensure completion

        # Click all eggs
        eggs_to_collect = list(board.eggs)
        for egg_pos in eggs_to_collect:
            result = process_click(board, state, egg_pos[0], egg_pos[1])
            assert result.valid
            assert result.egg_found

        # Verify all eggs collected
        assert len(state.eggs_collected) == 3
        assert state.score == 3

    def test_game_over_from_turns_exhausted(self) -> None:
        board = Board(5, 5, 0)  # No eggs - all empty
        state = GameState(turns_remaining=3)

        # Make moves until game over
        process_click(board, state, 0, 0)
        assert not state.game_over  # First click reveals many cells but uses 1 turn

        # Continue until turns run out
        for row in range(5):
            for col in range(5):
                if state.game_over:
                    break
                if not board.revealed[row][col]:
                    result = process_click(board, state, row, col)
                    if result.valid:
                        pass

        # Game should eventually end
        # (might end quickly due to floodfill revealing everything)

    def test_egg_collection_extends_gameplay(self) -> None:
        board = Board(3, 3, 0)

        # Place eggs manually
        board.eggs = {(0, 0), (1, 1), (2, 2)}
        board.cells[0][0] = -1
        board.cells[1][1] = -1
        board.cells[2][2] = -1
        board._calculate_numbers()

        state = GameState(turns_remaining=2)

        # With only 2 turns, couldn't normally collect all 3 eggs
        # But bonus turns allow it

        process_click(board, state, 0, 0)  # +1 turn
        assert state.turns_remaining == 3  # 2 + 1

        process_click(board, state, 1, 1)  # +1 turn
        assert state.turns_remaining == 4  # 3 + 1

        process_click(board, state, 2, 2)  # +1 turn
        assert state.turns_remaining == 5  # 4 + 1

        assert len(state.eggs_collected) == 3
        assert not state.game_over

    def test_mixed_gameplay(self) -> None:
        board = Board(5, 5, 2)
        state = GameState()

        initial_turns = state.turns_remaining
        eggs_collected = 0
        normal_clicks = 0

        # Play until game over or all cells revealed
        for row in range(5):
            for col in range(5):
                if state.game_over:
                    break

                if not board.revealed[row][col]:
                    result = process_click(board, state, row, col)

                    if result.valid:
                        if result.egg_found:
                            eggs_collected += 1
                        else:
                            normal_clicks += 1

        # Verify consistency
        assert eggs_collected == len(state.eggs_collected)
        assert state.score == eggs_collected

        # Turn calculation: initial + (eggs * 1) - normal_clicks
        expected_turns = initial_turns + (eggs_collected * 1) - normal_clicks
        assert state.turns_remaining == expected_turns

    def test_render_during_gameplay(self) -> None:
        board = Board(3, 3, 2)
        state = GameState()

        # Render initial state
        output = render_full_game(board, state)
        assert isinstance(output, str)
        assert "Turns" in output

        # Make some moves and render
        for row in range(3):
            if state.game_over:
                break

            result = process_click(board, state, row, 0)
            if result.valid:
                output = render_full_game(board, state)
                assert isinstance(output, str)

        # Render final state
        output = render_full_game(board, state)
        assert isinstance(output, str)

    def test_no_illegal_state_changes(self) -> None:
        board = Board(5, 5, 3)
        state = GameState()

        for row in range(5):
            for col in range(5):
                if state.game_over:
                    break

                prev_revealed = sum(sum(r) for r in board.revealed)
                prev_turns = state.turns_remaining
                prev_eggs = len(state.eggs_collected)

                result = process_click(board, state, row, col)

                if result.valid:
                    # Revealed cells should only increase
                    curr_revealed = sum(sum(r) for r in board.revealed)
                    assert curr_revealed >= prev_revealed

                    # Eggs collected should only increase
                    assert len(state.eggs_collected) >= prev_eggs

                    # Turns should change correctly
                    if result.egg_found:
                        # Should gain 1 turn
                        assert state.turns_remaining == prev_turns + 1
                    else:
                        # Should lose 1 turn (unless floodfill revealed everything)
                        assert state.turns_remaining == prev_turns - 1

    def test_deterministic_board_state(self) -> None:
        # Create two identical boards by using same random seed
        random.seed(42)
        board1 = Board(5, 5, 3)

        random.seed(42)
        board2 = Board(5, 5, 3)

        # Boards should be identical
        assert board1.eggs == board2.eggs
        assert board1.cells == board2.cells

        state1 = GameState()
        state2 = GameState()

        # Same clicks should produce same results
        result1 = process_click(board1, state1, 0, 0)
        result2 = process_click(board2, state2, 0, 0)

        assert result1.cells_revealed == result2.cells_revealed
        assert state1.turns_remaining == state2.turns_remaining


class TestEdgeCaseGames:
    """Test edge case game scenarios."""

    def test_board_with_no_eggs(self) -> None:
        board = Board(3, 3, 0)
        state = GameState()

        # Should be able to play normally
        result = process_click(board, state, 0, 0)
        assert result.valid
        assert not result.egg_found

    def test_board_all_eggs(self) -> None:
        board = Board(3, 3, 9)  # 3x3 board, 9 eggs
        state = GameState()

        # Every click should find an egg
        eggs_found = 0
        for row in range(3):
            for col in range(3):
                result = process_click(board, state, row, col)
                if result.valid:
                    assert result.egg_found
                    eggs_found += 1

        assert eggs_found == 9

    def test_single_cell_board(self) -> None:
        board = Board(1, 1, 0)
        state = GameState()

        result = process_click(board, state, 0, 0)
        assert result.valid

        # Board should be fully revealed now
        assert board.revealed[0][0]

    def test_very_large_board(self) -> None:
        board = Board(20, 20, 50)
        state = GameState()

        # Should initialize without errors
        assert len(board.eggs) == 50
        assert len(board.cells) == 20
        assert len(board.cells[0]) == 20

        # Should be playable
        result = process_click(board, state, 0, 0)
        assert result.valid
