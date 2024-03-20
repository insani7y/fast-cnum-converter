use pyo3::{create_exception, pymodule, types::PyModule, PyErr, PyResult, Python};

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

#[pymodule]
#[pyo3(name = "exceptions")]
#[allow(clippy::missing_errors_doc)]
pub fn exceptions_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("CnumConverterError", py.get_type::<CnumConverterError>())?;
    m.add(
        "ConvertStrToIntCNUMError",
        py.get_type::<ConvertStrToIntCNUMError>(),
    )?;
    m.add(
        "OverflowNumericCNUMError",
        py.get_type::<OverflowNumericCNUMError>(),
    )?;
    m.add(
        "NonPositiveNumericCNUMError",
        py.get_type::<NonPositiveNumericCNUMError>(),
    )?;
    m.add(
        "NotInAlphaNumericCNUMFormatError",
        py.get_type::<NotInAlphaNumericCNUMFormatError>(),
    )?;
    m.add(
        "BannedSymbolsInAlphaNumericCNUMError",
        py.get_type::<BannedSymbolsInAlphaNumericCNUMError>(),
    )?;
    Ok(())
}
