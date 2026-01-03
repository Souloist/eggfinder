"""Unit tests for board.py (Board)."""

import unittest

from eggfinder.board import Board


class TestBoard(unittest.TestCase):
    def test_default_initialization(self):
        board = Board()
        self.assertEqual(board.width, 10)
        self.assertEqual(board.height, 10)
        self.assertEqual(board.egg_count, 10)

    def test_custom_dimensions(self):
        board = Board(width=5, height=8, egg_count=3)
        self.assertEqual(board.width, 5)
        self.assertEqual(board.height, 8)
        self.assertEqual(board.egg_count, 3)

    def test_board_cells_initialized(self):
        board = Board(5, 5, 2)
        self.assertEqual(len(board.cells), 5)
        self.assertEqual(len(board.cells[0]), 5)

    def test_revealed_array_initialized(self):
        board = Board(5, 5, 2)
        for row in board.revealed:
            for cell in row:
                self.assertFalse(cell)

    def test_egg_placement_count(self):
        board = Board(10, 10, 15)
        self.assertEqual(len(board.eggs), 15)

        egg_count = sum(1 for row in board.cells for cell in row if cell == -1)
        self.assertEqual(egg_count, 15)

    def test_egg_placement_uniqueness(self):
        board = Board(5, 5, 5)
        self.assertEqual(len(board.eggs), 5)

    def test_is_valid_position(self):
        board = Board(5, 5, 2)

        self.assertTrue(board.is_valid_position(0, 0))
        self.assertTrue(board.is_valid_position(4, 4))
        self.assertTrue(board.is_valid_position(2, 3))

        self.assertFalse(board.is_valid_position(-1, 0))
        self.assertFalse(board.is_valid_position(0, -1))
        self.assertFalse(board.is_valid_position(5, 0))
        self.assertFalse(board.is_valid_position(0, 5))
        self.assertFalse(board.is_valid_position(10, 10))

    def test_get_cell_valid(self):
        board = Board(5, 5, 0)
        cell = board.get_cell(0, 0)
        self.assertIsNotNone(cell)
        self.assertIn(cell, range(-1, 9))

    def test_get_cell_invalid(self):
        board = Board(5, 5, 2)
        self.assertIsNone(board.get_cell(-1, 0))
        self.assertIsNone(board.get_cell(0, -1))
        self.assertIsNone(board.get_cell(5, 0))
        self.assertIsNone(board.get_cell(0, 5))

    def test_is_egg(self):
        board = Board(5, 5, 3)

        for egg_pos in board.eggs:
            self.assertTrue(board.is_egg(egg_pos[0], egg_pos[1]))

    def test_is_revealed(self):
        board = Board(5, 5, 2)

        self.assertFalse(board.is_revealed(0, 0))

        board.revealed[2][3] = True
        self.assertTrue(board.is_revealed(2, 3))

        self.assertFalse(board.is_revealed(-1, 0))
        self.assertFalse(board.is_revealed(5, 5))

    def test_get_neighbors(self):
        board = Board(5, 5, 0)

        neighbors = board.get_neighbors(0, 0)
        self.assertEqual(len(neighbors), 3)
        expected = {(0, 1), (1, 0), (1, 1)}
        self.assertEqual(set(neighbors), expected)

        neighbors = board.get_neighbors(0, 2)
        self.assertEqual(len(neighbors), 5)

        neighbors = board.get_neighbors(2, 2)
        self.assertEqual(len(neighbors), 8)

    def test_number_calculation_no_eggs(self):
        board = Board(5, 5, 0)
        for row in board.cells:
            for cell in row:
                self.assertEqual(cell, 0)

    def test_number_calculation_with_eggs(self):
        board = Board(5, 5, 2)

        for row in range(board.height):
            for col in range(board.width):
                cell_value = board.cells[row][col]
                if cell_value != -1:
                    self.assertIn(cell_value, range(0, 9))

                    actual_count = 0
                    for nr, nc in board.get_neighbors(row, col):
                        if board.is_egg(nr, nc):
                            actual_count += 1

                    self.assertEqual(
                        cell_value,
                        actual_count,
                        f"Cell ({row},{col}) should have {actual_count} but has {cell_value}",
                    )


class TestBoardDeterministic(unittest.TestCase):
    """Test Board with controlled egg placement for deterministic testing."""

    def test_single_egg_center(self):
        board = Board(3, 3, 0)

        board.eggs.add((1, 1))
        board.cells[1][1] = -1
        board._calculate_numbers()

        expected = [[1, 1, 1], [1, -1, 1], [1, 1, 1]]

        for row in range(3):
            for col in range(3):
                self.assertEqual(
                    board.cells[row][col], expected[row][col], f"Mismatch at ({row},{col})"
                )

    def test_two_adjacent_eggs(self):
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

        self.assertEqual(board.cells[0][0], 2)
        self.assertEqual(board.cells[0][1], 2)
        self.assertEqual(board.cells[1][2], 1)


if __name__ == "__main__":
    unittest.main()
