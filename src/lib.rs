pub mod common;
pub mod consts;
pub mod helpers;
pub mod python_exceptions;

use common::add_module;
use consts::{
    ANTI_OVERFLOW_FOR_BASE_OF_CNUMA, BASE_OF_CNUMA_NOTATION, CNUMA_POSSIBLE_SYMBOLS,
    CNUMA_POSSIBLE_SYMBOLS_STRING, CNUM_LENGTH, CNUM_MAXIMUM, FIRST_6_DIGITS_NUMBER,
    SEVEN_DIGITS_CNUM_MAX, SIX_DIGITS_CNUM_MAX, ZERO_ASCII_CODE,
};
use helpers::{
    check_str_is_in_cnum_format, convert_str_to_int, first_two_digits, validate_banned_symbols,
};
use pyo3::{prelude::*, types::PyString};
use python_exceptions::{
    exceptions_module, CnumConverterError, FastCnumConverterResult, NonPositiveNumericCNUMError,
    NotInAlphaNumericCNUMFormatError, OverflowNumericCNUMError,
};

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn inner_convert_cnum_to_alpha(
    numeric_cnum: i64,
    check_banned_symbols: bool,
) -> FastCnumConverterResult<String> {
    if numeric_cnum <= 0 {
        return Err(NonPositiveNumericCNUMError::new_err((
            "CNUM should be positive",
        )));
    }

    if numeric_cnum <= SIX_DIGITS_CNUM_MAX {
        return Ok(format!("{numeric_cnum:06}"));
    }

    if numeric_cnum <= SEVEN_DIGITS_CNUM_MAX {
        let first_two_digits = first_two_digits(numeric_cnum) as u32;

        return match std::char::from_u32(first_two_digits + ZERO_ASCII_CODE) {
            Some(result_char) => Ok(format!("{}{}", result_char, &numeric_cnum.to_string()[2..])),
            None => Err(CnumConverterError::new_err(("Invalid unicode point",))),
        };
    }

    if numeric_cnum > CNUM_MAXIMUM {
        return Err(OverflowNumericCNUMError::new_err((format!(
            "Overflow of numeric CNUM, currently maximum that we support is {CNUM_MAXIMUM}"
        ),)));
    }

    let mut mut_numeric_cnum = numeric_cnum + ANTI_OVERFLOW_FOR_BASE_OF_CNUMA;
    let mut result_cnum: String = String::with_capacity(CNUM_LENGTH);
    while mut_numeric_cnum > 0 {
        let index = (mut_numeric_cnum % BASE_OF_CNUMA_NOTATION) as usize;
        mut_numeric_cnum /= BASE_OF_CNUMA_NOTATION;
        let calculated_char = match CNUMA_POSSIBLE_SYMBOLS.get(index) {
            Some(cnum_character) => Ok(cnum_character),
            None => Err(CnumConverterError::new_err((format!(
                "Error getting {index}-th symbol from possible symbols."
            ),))),
        }?;
        result_cnum.insert(0, *calculated_char);
    }

    if check_banned_symbols {
        validate_banned_symbols(&result_cnum)?;
    }

    Ok(result_cnum)
}

#[pyfunction]
#[allow(clippy::missing_errors_doc)]
#[pyo3(signature = (source_maybe_cnum, check_banned_symbols=None))]
pub fn convert_cnum_to_alpha(
    source_maybe_cnum: &Bound<PyAny>,
    check_banned_symbols: Option<bool>,
) -> FastCnumConverterResult<String> {
    let is_check_banned_symbols = check_banned_symbols.unwrap_or(false);
    if source_maybe_cnum.is_instance_of::<PyString>() {
        let maybe_alphanumeric_cnum = &source_maybe_cnum.to_string().to_uppercase();
        if check_str_is_in_cnum_format(maybe_alphanumeric_cnum) {
            Ok(String::from(maybe_alphanumeric_cnum))
        } else {
            inner_convert_cnum_to_alpha(
                convert_str_to_int(maybe_alphanumeric_cnum)?,
                is_check_banned_symbols,
            )
        }
    } else if let Ok(numeric_cnum) = source_maybe_cnum.extract::<i64>() {
        inner_convert_cnum_to_alpha(numeric_cnum, is_check_banned_symbols)
    } else {
        Err(CnumConverterError::new_err((
            "Only str and int are acceptable.",
        )))
    }
}

#[pyfunction]
#[allow(
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
#[pyo3(signature = (source_maybe_cnum, check_banned_symbols=None))]
pub fn convert_cnum_to_numeric(
    source_maybe_cnum: &Bound<PyAny>,
    check_banned_symbols: Option<bool>,
) -> FastCnumConverterResult<i64> {
    if source_maybe_cnum.is_instance_of::<PyString>() {
        let string_cnum = source_maybe_cnum.to_string().to_uppercase();
        if let Ok(numeric_cnum) = string_cnum.parse::<i64>() {
            return Ok(numeric_cnum);
        }

        if !check_str_is_in_cnum_format(&string_cnum) {
            return Err(NotInAlphaNumericCNUMFormatError::new_err((
                "Sorry, your input string is not CNUM according to our symbols map.",
            )));
        }

        if check_banned_symbols.unwrap_or(false) {
            validate_banned_symbols(&string_cnum)?;
        }

        let (first_char, remainder) = string_cnum.split_at(1);
        if first_char.is_ascii() && remainder.chars().all(char::is_numeric) {
            let remainder_value: i64 = remainder.parse()?;
            let numeric_char_value = CNUMA_POSSIBLE_SYMBOLS_STRING.find(first_char).ok_or(
                NotInAlphaNumericCNUMFormatError::new_err((
                    "Sorry, your input string is not CNUM according to our symbols map.",
                )),
            )? as i64;
            return Ok(numeric_char_value * FIRST_6_DIGITS_NUMBER + remainder_value);
        }

        match i64::from_str_radix(&string_cnum, BASE_OF_CNUMA_NOTATION as u32) {
            Ok(number) => Ok(number - ANTI_OVERFLOW_FOR_BASE_OF_CNUMA),
            Err(err) => Err(err.into()),
        }
    } else if let Ok(numeric_cnum) = source_maybe_cnum.extract::<i64>() {
        Ok(numeric_cnum)
    } else {
        Err(CnumConverterError::new_err((
            "Only str and int are acceptable.",
        )))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_internal")]
fn fast_cnum_converter(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    add_module(py, m, "exceptions", exceptions_module)?;
    m.add_function(wrap_pyfunction!(convert_cnum_to_alpha, m)?)?;
    m.add_function(wrap_pyfunction!(convert_cnum_to_numeric, m)?)?;
    Ok(())
}
