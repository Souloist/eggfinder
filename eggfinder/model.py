from pydantic import BaseModel, Field

from .constants import GameConfig
from .types import Coordinate


class GameState(BaseModel):
    """Pure game state - no logic, just data."""

    turns_remaining: int = GameConfig.DEFAULT_TURNS
    eggs_collected: set[Coordinate] = Field(default_factory=set)
    game_over: bool = False
    score: int = 0
