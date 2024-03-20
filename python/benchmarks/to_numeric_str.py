import datetime
import random

import cnum_converter

import fast_cnum_converter


# 711566042, "LOVEU2"
def generate_string_cnum() -> str:
    return "".join([random.choice("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789") for _ in range(6)])  # noqa: S311


def main() -> None:
    amount = 1000000
    cnum_array = [generate_string_cnum() for _ in range(amount)]
    t1 = datetime.datetime.now()  # noqa: DTZ005
    for cnum in cnum_array:
        fast_cnum_converter.convert_cnum_to_numeric(cnum)

    print(f"Rust took: {datetime.datetime.now()-t1}s for {amount} generations.")  # noqa: DTZ005, T201

    t1 = datetime.datetime.now()  # noqa: DTZ005
    for cnum in cnum_array:
        cnum_converter.convert_cnum_to_numeric(cnum)

    print(f"Python took: {datetime.datetime.now()-t1}s for {amount} generations.")  # noqa: DTZ005, T201


if __name__ == "__main__":
    main()
