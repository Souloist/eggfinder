"""EggFinder interactive CLI game."""

import argparse
import sys
from eggfinder import Board, GameState, process_click, render_full_game


def parse_args():
    parser = argparse.ArgumentParser(
        description='EggFinder - A minesweeper-like game where you collect eggs!',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Game Rules:
  - Start with 10 turns
  - Click cells to reveal them (costs 1 turn)
  - Click an egg to collect it (grants +2 bonus turns!)
  - Goal: Collect as many eggs as possible before turns run out

Display:
  * = collected egg    E = revealed egg    # = hidden cell
  . = empty cell       1-8 = adjacent egg count
        """
    )
    parser.add_argument('-W', '--width', type=int, default=10,
                        help='Board width (default: 10)')
    parser.add_argument('-H', '--height', type=int, default=10,
                        help='Board height (default: 10)')
    parser.add_argument('-e', '--eggs', type=int, default=10,
                        help='Number of eggs (default: 10)')
    return parser.parse_args()


def get_user_input():
    """Get user input for row and column. Returns (row, col) or None to quit."""
    while True:
        try:
            user_input = input("\nEnter coordinates (row col) or 'q' to quit: ").strip().lower()

            if user_input in ['q', 'quit', 'exit']:
                return None

            if user_input in ['h', 'help']:
                print("\nCommands:")
                print("  row col  - Click cell at (row, col), e.g., '3 5'")
                print("  q, quit  - Quit the game")
                print("  h, help  - Show this help message")
                continue

            parts = user_input.split()
            if len(parts) != 2:
                print("Invalid input. Please enter two numbers (row col) or 'q' to quit.")
                continue

            row = int(parts[0])
            col = int(parts[1])
            return (row, col)

        except ValueError:
            print("Invalid input. Please enter two numbers (row col) or 'q' to quit.")
        except (KeyboardInterrupt, EOFError):
            print("\n\nGame interrupted by user.")
            return None


def main():
    args = parse_args()

    print("=" * 60)
    print("        EggFinder - Collect eggs to gain turns!")
    print("=" * 60)
    print(f"\nBoard: {args.width}x{args.height} with {args.eggs} eggs")
    print("Type 'h' for help")
    print()

    board = Board(args.width, args.height, args.eggs)
    game_state = GameState()

    while not game_state.game_over:
        print(render_full_game(board, game_state))

        coords = get_user_input()
        if coords is None:
            print("\nThanks for playing!")
            print(f"Final Score: {game_state.score}/{board.egg_count} eggs collected")
            sys.exit(0)

        row, col = coords
        result = process_click(board, game_state, row, col)

        if result['valid']:
            print(f"\n✓ {result['message']}")
            if result['egg_found']:
                print("  Egg collected! +2 bonus turns!")
        else:
            print(f"\n✗ {result['message']}")

    print("\n" + render_full_game(board, game_state))
    print("\n" + "=" * 60)
    print("                      GAME OVER!")
    print("=" * 60)
    print(f"Final Score: {game_state.score}/{board.egg_count} eggs collected")

    if len(game_state.eggs_collected) == board.egg_count:
        print("Perfect! You collected all the eggs!")

    show_solution = input("\nShow egg locations? (y/n): ").strip().lower()
    if show_solution in ['y', 'yes']:
        print("\nBoard with all eggs revealed:")
        print(render_full_game(board, game_state, show_all=True))


if __name__ == "__main__":
    main()
