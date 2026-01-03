import random

from .constants import CellType, GameConfig
from .exceptions import InvalidBoardError


class Board:
    """Board state and operations."""

    def __init__(
        self,
        width: int = GameConfig.DEFAULT_BOARD_WIDTH,
        height: int = GameConfig.DEFAULT_BOARD_HEIGHT,
        egg_count: int = GameConfig.DEFAULT_EGG_COUNT,
    ):
        if width <= 0 or height <= 0:
            raise InvalidBoardError(f"Board dimensions must be positive, got {width}x{height}")

        max_eggs = width * height
        if egg_count < 0 or egg_count > max_eggs:
            raise InvalidBoardError(f"Egg count must be between 0 and {max_eggs}, got {egg_count}")

        self.width = width
        self.height = height
        self.egg_count = egg_count
        self.cells: list[list[int]] = []
        self.revealed: list[list[bool]] = []
        self.eggs: set[tuple[int, int]] = set()

        self._initialize()
        self._place_eggs()
        self._calculate_numbers()

    def _initialize(self) -> None:
        self.cells = [[0 for _ in range(self.width)] for _ in range(self.height)]
        self.revealed = [[False for _ in range(self.width)] for _ in range(self.height)]

    def _place_eggs(self) -> None:
        placed = 0
        while placed < self.egg_count:
            row = random.randint(0, self.height - 1)
            col = random.randint(0, self.width - 1)

            if (row, col) not in self.eggs:
                self.eggs.add((row, col))
                self.cells[row][col] = CellType.EGG
                placed += 1

    def _calculate_numbers(self) -> None:
        """Calculate adjacent egg counts for each cell."""
        for row in range(self.height):
            for col in range(self.width):
                if self.cells[row][col] != CellType.EGG:
                    self.cells[row][col] = self._count_adjacent_eggs(row, col)

    def _count_adjacent_eggs(self, row: int, col: int) -> int:
        """Count eggs adjacent to the given position."""
        return sum(1 for nr, nc in self.get_neighbors(row, col) if self.is_egg(nr, nc))

    def is_valid_position(self, row: int, col: int) -> bool:
        return 0 <= row < self.height and 0 <= col < self.width

    def get_cell(self, row: int, col: int) -> int | None:
        if not self.is_valid_position(row, col):
            return None
        return self.cells[row][col]

    def is_egg(self, row: int, col: int) -> bool:
        return (row, col) in self.eggs

    def is_revealed(self, row: int, col: int) -> bool:
        if not self.is_valid_position(row, col):
            return False
        return self.revealed[row][col]

    def get_neighbors(self, row: int, col: int) -> list[tuple[int, int]]:
        neighbors = []
        for dr in [-1, 0, 1]:
            for dc in [-1, 0, 1]:
                if dr == 0 and dc == 0:
                    continue
                nr, nc = row + dr, col + dc
                if self.is_valid_position(nr, nc):
                    neighbors.append((nr, nc))
        return neighbors
