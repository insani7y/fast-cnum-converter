import pytest

import fast_cnum_converter
from fast_cnum_converter import exceptions
from tests.const import CNUM_MAXIMUM


@pytest.mark.parametrize("input_value", [CNUM_MAXIMUM + 1])
def test_convert_cnum_to_alpha__numeric_overflow(input_value: int) -> None:
    with pytest.raises(exceptions.OverflowNumericCNUMError):
        fast_cnum_converter.convert_cnum_to_alpha(fast_cnum_converter.convert_cnum_to_alpha(input_value))


@pytest.mark.parametrize("input_value", [0, -1, -100_000_001])
def test_alpha_to_alpha__nonpositive_numeric(input_value: int) -> None:
    with pytest.raises(exceptions.NonPositiveNumericCNUMError):
        fast_cnum_converter.convert_cnum_to_numeric(fast_cnum_converter.convert_cnum_to_alpha(input_value))


@pytest.mark.parametrize("input_value", ["1TN4WNUBLJ5", ""])
def test_convert_cnum_to_alpha__invalid_alpha_format(input_value: str) -> None:
    with pytest.raises(exceptions.NotInAlphaNumericCNUMFormatError):
        fast_cnum_converter.convert_cnum_to_alpha(fast_cnum_converter.convert_cnum_to_numeric(input_value))


@pytest.mark.parametrize("input_value", ["ABCKEKESTAN", "STUPID   (not real)", "                  ", ""])
def test_convert_cnum_to_alpha__nonint_strings(input_value: str) -> None:
    with pytest.raises(exceptions.ConvertStrToIntCNUMError):
        fast_cnum_converter.convert_cnum_to_alpha(input_value)


@pytest.mark.parametrize("input_value", ["STUPID", "LOVEU2"])
def test_convert_cnum_to_numeric__banned_symbols(input_value: str) -> None:
    with pytest.raises(exceptions.BannedSymbolsInAlphaNumericCNUMError):
        fast_cnum_converter.convert_cnum_to_numeric(input_value, check_banned_symbols=True)


@pytest.mark.parametrize("input_value", [1143194533, 711566042])
def test_convert_cnum_to_alpha__banned_symbols(input_value: int) -> None:
    with pytest.raises(exceptions.BannedSymbolsInAlphaNumericCNUMError):
        fast_cnum_converter.convert_cnum_to_alpha(input_value, check_banned_symbols=True)
