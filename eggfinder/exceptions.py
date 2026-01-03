"""Custom exceptions for EggFinder."""


class EggFinderError(Exception):
    """Base exception for all EggFinder errors."""



class InvalidBoardError(EggFinderError):
    """Raised when board parameters are invalid."""



class InvalidMoveError(EggFinderError):
    """Raised when a move is invalid."""



class GameOverError(EggFinderError):
    """Raised when attempting to play after game is over."""

