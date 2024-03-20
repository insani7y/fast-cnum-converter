import datetime
import random

import cnum_converter

import fast_cnum_converter


def main() -> None:
    amount = 1000000
    t1 = datetime.datetime.now()  # noqa: DTZ005

    for _ in range(amount):
        fast_cnum_converter.convert_cnum_to_alpha(str(random.randrange(1, 1_576_782_335)))  # noqa: S311

    print(f"Rust took: {datetime.datetime.now()-t1}s for {amount} generations.")  # noqa: DTZ005, T201

    t1 = datetime.datetime.now()  # noqa: DTZ005

    for _ in range(amount):
        cnum_converter.convert_cnum_to_alpha(str(random.randrange(1, 1_576_782_335)))  # noqa: S311

    print(f"Python took: {datetime.datetime.now()-t1}s for {amount} generations.")  # noqa: DTZ005, T201


if __name__ == "__main__":
    main()
