import dataclasses

import pytest

import fast_cnum_converter


@dataclasses.dataclass
class CnumPair:
    number: int
    alpha: str


GOOD_PAIRS = [
    CnumPair(223, "000223"),
    CnumPair(204, "000204"),
    CnumPair(201, "000201"),
    CnumPair(1959, "001959"),
    CnumPair(6710, "006710"),
    CnumPair(3772, "003772"),
    CnumPair(70647, "070647"),
    CnumPair(86142, "086142"),
    CnumPair(48706, "048706"),
    CnumPair(800801, "800801"),
    CnumPair(279309, "279309"),
    CnumPair(167211, "167211"),
    CnumPair(3005556, "U05556"),
    CnumPair(2801733, "S01733"),
    CnumPair(2942957, "T42957"),
    CnumPair(23834893, "ABEY31"),
    CnumPair(24321953, "ABPDWH"),
    CnumPair(22119064, "AAE654"),
    CnumPair(808064656, "NABPN4"),
    CnumPair(808061369, "NABN3T"),
    CnumPair(808116722, "NACTTE"),
    CnumPair(1000000, "A00000"),
    CnumPair(999999, "999999"),
    CnumPair(3599999, "Z99999"),
    CnumPair(3600000, "9ZD8QO"),
    CnumPair(1576782335, "ZZZZZZ"),
    CnumPair(1143194533, "STUPID"),
    CnumPair(711566042, "LOVEU2"),
    # вдали маячит какой-то зикож! зикож атаковал караван!
    # милорд, крестьяне волнуются — приближается зикож! зикожем единым сыт не будешь!
    CnumPair(1547483647, "ZIK0ZJ"),
]


@pytest.mark.parametrize("input_value", GOOD_PAIRS)
def test_basic_from_numeric_to_alpha(input_value: CnumPair) -> None:
    assert fast_cnum_converter.convert_cnum_to_alpha(input_value.number) == input_value.alpha


@pytest.mark.parametrize("input_value", GOOD_PAIRS)
def test_basic_from_alpha_to_numeric(input_value: CnumPair) -> None:
    assert fast_cnum_converter.convert_cnum_to_numeric(input_value.alpha.upper()) == input_value.number  # type: ignore[comparison-overlap]
    assert fast_cnum_converter.convert_cnum_to_numeric(input_value.alpha.lower()) == input_value.number  # type: ignore[comparison-overlap]


@pytest.mark.parametrize("input_value", GOOD_PAIRS)
def test_cycled_number(input_value: CnumPair) -> None:
    assert (  # type: ignore[comparison-overlap]
        fast_cnum_converter.convert_cnum_to_numeric(fast_cnum_converter.convert_cnum_to_alpha(input_value.number))
    ) == input_value.number


@pytest.mark.parametrize("input_value", GOOD_PAIRS)
def test_cycled_alpha(input_value: CnumPair) -> None:
    assert (
        fast_cnum_converter.convert_cnum_to_alpha(
            fast_cnum_converter.convert_cnum_to_numeric(input_value.alpha.lower()),
        )
    ) == input_value.alpha
    assert (
        fast_cnum_converter.convert_cnum_to_alpha(fast_cnum_converter.convert_cnum_to_numeric(input_value.alpha))
    ) == input_value.alpha


@pytest.mark.parametrize("input_value", GOOD_PAIRS)
def test_alpha_to_alpha(input_value: CnumPair) -> None:
    assert fast_cnum_converter.convert_cnum_to_alpha(input_value.alpha) == input_value.alpha


@pytest.mark.parametrize("input_value", GOOD_PAIRS)
def test_numeric_to_numeric(input_value: CnumPair) -> None:
    assert fast_cnum_converter.convert_cnum_to_numeric(input_value.number) == input_value.number  # type: ignore[comparison-overlap]
