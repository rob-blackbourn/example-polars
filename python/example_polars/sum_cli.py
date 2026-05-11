import argparse

from ._lib import sum_as_string


def main() -> None:
    parser = argparse.ArgumentParser("sum 2 integers")
    parser.add_argument("x", type=int, help="first integer")
    parser.add_argument("y", type=int, help="second integer")
    args = parser.parse_args()
    result = sum_as_string(args.x, args.y)
    print(f"{args.x} + {args.y} = {result}")


if __name__ == "__main__":
    main()
