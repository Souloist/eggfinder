"""Unit tests for board.py (Board)."""

from eggfinder.board import Board


class TestBoard:
    def test_default_initialization(self) -> None:
        board = Board()
        assert board.width == 10
        assert board.height == 10
        assert board.egg_count == 10

    def test_custom_dimensions(self) -> None:
        board = Board(width=5, height=8, egg_count=3)
        assert board.width == 5
        assert board.height == 8
        assert board.egg_count == 3

    def test_board_cells_initialized(self) -> None:
        board = Board(5, 5, 2)
        assert len(board.cells) == 5
        assert len(board.cells[0]) == 5

    def test_revealed_array_initialized(self) -> None:
        board = Board(5, 5, 2)
        for row in board.revealed:
            for cell in row:
                assert not cell

    def test_egg_placement_count(self) -> None:
        board = Board(10, 10, 15)
        assert len(board.eggs) == 15

        egg_count = sum(1 for row in board.cells for cell in row if cell == -1)
        assert egg_count == 15

    def test_egg_placement_uniqueness(self) -> None:
        board = Board(5, 5, 5)
        assert len(board.eggs) == 5

    def test_is_valid_position(self) -> None:
        board = Board(5, 5, 2)

        assert board.is_valid_position(0, 0)
        assert board.is_valid_position(4, 4)
        assert board.is_valid_position(2, 3)

        assert not board.is_valid_position(-1, 0)
        assert not board.is_valid_position(0, -1)
        assert not board.is_valid_position(5, 0)
        assert not board.is_valid_position(0, 5)
        assert not board.is_valid_position(10, 10)

    def test_get_cell_valid(self) -> None:
        board = Board(5, 5, 0)
        cell = board.get_cell(0, 0)
        assert cell is not None
        assert cell in range(-1, 9)

    def test_get_cell_invalid(self) -> None:
        board = Board(5, 5, 2)
        assert board.get_cell(-1, 0) is None
        assert board.get_cell(0, -1) is None
        assert board.get_cell(5, 0) is None
        assert board.get_cell(0, 5) is None

    def test_is_egg(self) -> None:
        board = Board(5, 5, 3)

        for egg_pos in board.eggs:
            assert board.is_egg(egg_pos[0], egg_pos[1])

    def test_is_revealed(self) -> None:
        board = Board(5, 5, 2)

        assert not board.is_revealed(0, 0)

        board.revealed[2][3] = True
        assert board.is_revealed(2, 3)

        assert not board.is_revealed(-1, 0)
        assert not board.is_revealed(5, 5)

    def test_get_neighbors(self) -> None:
        board = Board(5, 5, 0)

        neighbors = board.get_neighbors(0, 0)
        assert len(neighbors) == 3
        expected = {(0, 1), (1, 0), (1, 1)}
        assert set(neighbors) == expected

        neighbors = board.get_neighbors(0, 2)
        assert len(neighbors) == 5

        neighbors = board.get_neighbors(2, 2)
        assert len(neighbors) == 8

    def test_number_calculation_no_eggs(self) -> None:
        board = Board(5, 5, 0)
        for row in board.cells:
            for cell in row:
                assert cell == 0

    def test_number_calculation_with_eggs(self) -> None:
        board = Board(5, 5, 2)

        for row in range(board.height):
            for col in range(board.width):
                cell_value = board.cells[row][col]
                if cell_value != -1:
                    assert cell_value in range(9)

                    actual_count = 0
                    for nr, nc in board.get_neighbors(row, col):
                        if board.is_egg(nr, nc):
                            actual_count += 1

                    assert cell_value == actual_count, (
                        f"Cell ({row},{col}) should have {actual_count} but has {cell_value}"
                    )


class TestBoardDeterministic:
    """Test Board with controlled egg placement for deterministic testing."""

    def test_single_egg_center(self) -> None:
        board = Board(3, 3, 0)

        board.eggs.add((1, 1))
        board.cells[1][1] = -1
        board._calculate_numbers()

        expected = [[1, 1, 1], [1, -1, 1], [1, 1, 1]]

        for row in range(3):
            for col in range(3):
                assert board.cells[row][col] == expected[row][col], f"Mismatch at ({row},{col})"

    def test_two_adjacent_eggs(self) -> None:
        board = Board(3, 3, 0)

        # Layout:
        #   0 1 2
        # 0 2 2 1
        # 1 E E 1
        # 2 1 1 .
        board.eggs.add((1, 0))
        board.eggs.add((1, 1))
        board.cells[1][0] = -1
        board.cells[1][1] = -1
        board._calculate_numbers()

        assert board.cells[0][0] == 2
        assert board.cells[0][1] == 2
        assert board.cells[1][2] == 1
