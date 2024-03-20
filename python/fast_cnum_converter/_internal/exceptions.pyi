class CnumConverterError(Exception):
    """Base class for cnum converter exceptions."""

class ConvertStrToIntCNUMError(CnumConverterError):
    """Raises when cannot convert str to int."""

class NonPositiveNumericCNUMError(CnumConverterError):
    """Raises if negative cnum value was passed in."""

class OverflowNumericCNUMError(CnumConverterError):
    """Raises if too large cnum value was passed in."""

class NotInAlphaNumericCNUMFormatError(CnumConverterError):
    """Raises if non-alphanumeric cnum value was passed in."""

class BannedSymbolsInAlphaNumericCNUMError(CnumConverterError):
    """Raises when cnum contais banned symbols."""
