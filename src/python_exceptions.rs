use pyo3::{
    create_exception,
    types::{PyModule, PyModuleMethods},
    Bound, PyErr, PyResult, Python,
};

pub type FastCnumConverterResult<T> = Result<T, PyErr>;

create_exception!(
    fast_cnum_converter.exceptions,
    CnumConverterError,
    pyo3::exceptions::PyValueError
);
create_exception!(
    fast_cnum_converter.exceptions,
    ConvertStrToIntCNUMError,
    CnumConverterError
);
create_exception!(
    fast_cnum_converter.exceptions,
    OverflowNumericCNUMError,
    CnumConverterError
);
create_exception!(
    fast_cnum_converter.exceptions,
    NonPositiveNumericCNUMError,
    CnumConverterError
);
create_exception!(
    fast_cnum_converter.exceptions,
    NotInAlphaNumericCNUMFormatError,
    CnumConverterError
);
create_exception!(
    fast_cnum_converter.exceptions,
    BannedSymbolsInAlphaNumericCNUMError,
    CnumConverterError
);

#[allow(clippy::missing_errors_doc)]
pub fn exceptions_module(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add(
        "CnumConverterError",
        py.get_type_bound::<CnumConverterError>(),
    )?;
    m.add(
        "ConvertStrToIntCNUMError",
        py.get_type_bound::<ConvertStrToIntCNUMError>(),
    )?;
    m.add(
        "OverflowNumericCNUMError",
        py.get_type_bound::<OverflowNumericCNUMError>(),
    )?;
    m.add(
        "NonPositiveNumericCNUMError",
        py.get_type_bound::<NonPositiveNumericCNUMError>(),
    )?;
    m.add(
        "NotInAlphaNumericCNUMFormatError",
        py.get_type_bound::<NotInAlphaNumericCNUMFormatError>(),
    )?;
    m.add(
        "BannedSymbolsInAlphaNumericCNUMError",
        py.get_type_bound::<BannedSymbolsInAlphaNumericCNUMError>(),
    )?;
    Ok(())
}
