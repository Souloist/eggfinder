"""Custom exceptions for EggFinder."""


class EggFinderError(Exception):
    """Base exception for all EggFinder errors."""

    pass


class InvalidBoardError(EggFinderError):
    """Raised when board parameters are invalid."""

    pass


class InvalidMoveError(EggFinderError):
    """Raised when a move is invalid."""

    pass


class GameOverError(EggFinderError):
    """Raised when attempting to play after game is over."""

    pass
