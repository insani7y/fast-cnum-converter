use crate::{
    consts::{BANNED_SYMBOLS, CNUMA_POSSIBLE_SYMBOLS, CNUM_LENGTH},
    python_exceptions::{
        BannedSymbolsInAlphaNumericCNUMError, ConvertStrToIntCNUMError, FastCnumConverterResult,
    },
};

#[must_use]
pub fn first_two_digits(n: i64) -> i64 {
    let mut temp = n;
    while temp > 99 {
        temp /= 10;
    }
    temp
}

#[must_use]
pub fn check_str_is_in_cnum_format(maybe_cnum: &str) -> bool {
    for char in maybe_cnum.chars() {
        if !CNUMA_POSSIBLE_SYMBOLS.contains(&char) {
            return false;
        }
    }

    maybe_cnum.len() == CNUM_LENGTH
}

#[allow(clippy::missing_errors_doc)]
pub fn convert_str_to_int(some_string: &str) -> FastCnumConverterResult<i64> {
    match some_string.parse::<i64>() {
        Ok(parsed_int) => Ok(parsed_int),
        Err(_) => Err(ConvertStrToIntCNUMError::new_err((format!(
            "Your input string could not be converted to integer: {some_string}",
        ),))),
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn validate_banned_symbols(maybe_cnum_symbols: &str) -> FastCnumConverterResult<()> {
    for cnum_symbol in maybe_cnum_symbols.chars() {
        if BANNED_SYMBOLS.contains(&cnum_symbol) {
            return Err(BannedSymbolsInAlphaNumericCNUMError::new_err((format!(
                "CNUM {maybe_cnum_symbols} contains banned symbols: {cnum_symbol}"
            ),)));
        }
    }
    Ok(())
}
