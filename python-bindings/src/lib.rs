pub mod converter;
use pyo3::prelude::*;
use converter::{sum_as_string, factorial, read_json_configs};

/// A Python module implemented in Rust.
#[pymodule]
fn python_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    m.add_function(wrap_pyfunction!(read_json_configs, m)?)?;
    Ok(())
}
