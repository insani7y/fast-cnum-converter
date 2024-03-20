# Fast cnum converter

This a faster version of original cnum-converter, due to internal rust usage.

## Benchmarks

Here's some benchmarks to prove: rust usage leads to a higher performance.  
We are testing:

- convert_cnum_to_alpha with str passed in
- convert_cnum_to_alpha with int passed in
- convert_cnum_to_numeric with str passed in

Benchmarking time usage per 1_000_000 operations in seconds, so, `lower bar value - better`.

[![benchmark image](https://i.postimg.cc/Gt21qDw9/plot.png)](https://postimg.cc/KkdVYRYy)

As you can see - rust beats python on every test.  
You can find all benchmarks in `benchmarks` directory and test it yourself.

## Installation

This package can be installed using poetry or through PIP.

### Poetry

```bash
poetry add fast-cnum-converter
```

### PIP

```bash
pip3 install fast-cnum-converter
```

## Usage

Before using converter you need to know that:

- CNUMA is alphanumeric with base 36 by default(it can be changed via constant `BASE_OF_CNUMA_NOTATION` if needed)
- some of symbols is banned from using in CNUMA(see constant `BANNED_SYMBOLS`)
- length of CNUMA is always = 6 symbols
- CNUMA is case-insensitive and by default using in uppercase
- CNUM should be positive number

### To CNUMA

Convert to alphanumeric CNUM:

```python
from fast_cnum_converter import convert_cnum_to_alpha

convert_cnum_to_alpha(10020044)
# 'A36UH8'

convert_cnum_to_alpha(711566042)
# 'LOVEU2'
```

And as been said earlier some symbols for CNUMA is banned.
By default we don't do anything when see that CNUM is converting to CNUMA with this symbols.
But you can enable strict mode with option `check_banned_symbols` that will enable raising error on banned symbols in resulting CNUMA.

```python
from fast_cnum_converter import convert_cnum_to_alpha

convert_cnum_to_alpha(10020044, check_banned_symbols=True)
# 'A36UH8'

convert_cnum_to_alpha(711566042, check_banned_symbols=True)
# BannedSymbolsInAlphaNumericCNUMError
```

### To CNUM

Convert to numeric CNUM:

```python
from fast_cnum_converter import convert_cnum_to_numeric

convert_cnum_to_numeric("ABC123")
# 23698779

convert_cnum_to_numeric("LOVEU2")
# 711566042
```

As been said earlier some symbols for CNUMA is banned.
By default we don't do anything when see that passed CNUMA contains banned symbols.
And you can enable strict mode with option `check_banned_symbols` that will enable raising error on such CNUMA.

```python
from fast_cnum_converter import convert_cnum_to_alpha

convert_cnum_to_numeric("ABC123", check_banned_symbols=True)
# '23698779'

convert_cnum_to_numeric("LOVEU2", check_banned_symbols=True)
# BannedSymbolsInAlphaNumericCNUMError
```

## Errors

All errors of cnum_converter is based on `ValueError`.
Base error is `CnumConverterError` and others errors for concrete problems derived from it.

## Bonus

Heavy tests coverage, overflow cases, good cases, bad cases.
