use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod chudnovsky;


// Compute pi to a given precision.
#[pyfunction]
#[text_signature = "(prec, iter, /)"]
fn compute_pi(prec: u32, iter: i32) -> PyResult<String> {
    let pi = chudnovsky::compute_pi(prec, iter);
    Ok(pi)
}

#[pymodule]
fn chudnovsky(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(compute_pi))?;

    Ok(())
}
