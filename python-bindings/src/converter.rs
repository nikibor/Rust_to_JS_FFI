use pyo3::prelude::*;
use pyo3::types::PyList;
// use core::solvers::rayon_executor::RayonWorker;


/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// asd
#[pyfunction]
pub fn factorial(a: Vec<i32>) -> PyResult<String>{
    // let result = RayonWorker::sum_of_squares(a);
    // Ok(result.to_string())
    let x: i32 = a.iter().sum();
    Ok(x.to_string())
}