"""Type definitions for EggFinder."""

from dataclasses import dataclass
from typing import Self

# Type aliases for coordinate and grid types
type Coordinate = tuple[int, int]
type Grid[T] = list[list[T]]


@dataclass(frozen=True)
class ClickResult:
    """Result of processing a click action.

    Immutable result object that encapsulates all information about a click action.
    """

    valid: bool
    message: str
    egg_found: bool
    turns_delta: int
    cells_revealed: int
    game_over: bool

    @classmethod
    def invalid(cls, message: str, game_over: bool = False) -> Self:
        """Create an invalid click result."""
        return cls(
            valid=False,
            message=message,
            egg_found=False,
            turns_delta=0,
            cells_revealed=0,
            game_over=game_over,
        )

    @classmethod
    def egg_collected(cls, message: str, turns_delta: int, game_over: bool) -> Self:
        """Create a result for collecting an egg."""
        return cls(
            valid=True,
            message=message,
            egg_found=True,
            turns_delta=turns_delta,
            cells_revealed=1,
            game_over=game_over,
        )

    @classmethod
    def cell_revealed(cls, message: str, cells_revealed: int, game_over: bool) -> Self:
        """Create a result for revealing normal cells."""
        return cls(
            valid=True,
            message=message,
            egg_found=False,
            turns_delta=-1,
            cells_revealed=cells_revealed,
            game_over=game_over,
        )
