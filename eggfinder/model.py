from typing import Set, Tuple
from pydantic import BaseModel, Field


class GameState(BaseModel):
    """Pure game state - no logic, just data."""

    turns_remaining: int = 10
    eggs_collected: Set[Tuple[int, int]] = Field(default_factory=set)
    game_over: bool = False
    score: int = 0

    class Config:
        arbitrary_types_allowed = True
