"""Game constants and configuration."""

from enum import IntEnum, Enum


class GameConfig:
    """Game rule configuration."""
    DEFAULT_TURNS = 10
    EGG_BONUS_TURNS = 1
    DEFAULT_BOARD_WIDTH = 10
    DEFAULT_BOARD_HEIGHT = 10
    DEFAULT_EGG_COUNT = 10


class CellType(IntEnum):
    """Cell type constants for board representation."""
    EGG = -1
    EMPTY = 0


class CellDisplay(str, Enum):
    """Display symbols for board cells."""
    COLLECTED_EGG = "★"
    REVEALED_EGG = "○"
    HIDDEN = "■"
    EMPTY = "·"
