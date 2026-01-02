from typing import Set, Tuple
from pydantic import BaseModel, Field

from .constants import GameConfig


class GameState(BaseModel):
    """Pure game state - no logic, just data."""

    turns_remaining: int = GameConfig.DEFAULT_TURNS
    eggs_collected: Set[Tuple[int, int]] = Field(default_factory=set)
    game_over: bool = False
    score: int = 0

    class Config:
        arbitrary_types_allowed = True
