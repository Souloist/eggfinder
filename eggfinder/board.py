import random
from typing import Set, Tuple, List, Optional


class Board:
    """Board state and operations."""

    def __init__(self, width: int = 10, height: int = 10, egg_count: int = 10):
        self.width = width
        self.height = height
        self.egg_count = egg_count
        self.cells: List[List[int]] = []
        self.revealed: List[List[bool]] = []
        self.eggs: Set[Tuple[int, int]] = set()

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
                self.cells[row][col] = -1
                placed += 1

    def _calculate_numbers(self) -> None:
        """Calculate adjacent egg counts for each cell."""
        for row in range(self.height):
            for col in range(self.width):
                if self.cells[row][col] != -1:
                    count = 0
                    for dr in [-1, 0, 1]:
                        for dc in [-1, 0, 1]:
                            if dr == 0 and dc == 0:
                                continue
                            nr, nc = row + dr, col + dc
                            if self.is_valid_position(nr, nc):
                                if self.cells[nr][nc] == -1:
                                    count += 1
                    self.cells[row][col] = count

    def is_valid_position(self, row: int, col: int) -> bool:
        return 0 <= row < self.height and 0 <= col < self.width

    def get_cell(self, row: int, col: int) -> Optional[int]:
        if not self.is_valid_position(row, col):
            return None
        return self.cells[row][col]

    def is_egg(self, row: int, col: int) -> bool:
        return (row, col) in self.eggs

    def is_revealed(self, row: int, col: int) -> bool:
        if not self.is_valid_position(row, col):
            return False
        return self.revealed[row][col]

    def get_neighbors(self, row: int, col: int) -> List[Tuple[int, int]]:
        neighbors = []
        for dr in [-1, 0, 1]:
            for dc in [-1, 0, 1]:
                if dr == 0 and dc == 0:
                    continue
                nr, nc = row + dr, col + dc
                if self.is_valid_position(nr, nc):
                    neighbors.append((nr, nc))
        return neighbors
