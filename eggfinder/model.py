from pydantic import BaseModel, ConfigDict, Field

from .constants import GameConfig


class GameState(BaseModel):
    """Pure game state - no logic, just data."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    turns_remaining: int = GameConfig.DEFAULT_TURNS
    eggs_collected: set[tuple[int, int]] = Field(default_factory=set)
    game_over: bool = False
    score: int = 0
