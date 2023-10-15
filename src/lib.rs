mod lorenz;
mod three_body;

use pyo3::prelude::*;

use crate::lorenz::solve_lorenz;
use crate::three_body::{list_sum, all_planet_acc_nbody_py, total_energy_nbody_py};

/// A Python module implemented in Rust.
#[pymodule]
fn nbodysolver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_lorenz, m)?)?;
    m.add_function(wrap_pyfunction!(list_sum, m)?)?;
    m.add_function(wrap_pyfunction!(all_planet_acc_nbody_py, m)?)?;
    m.add_function(wrap_pyfunction!(total_energy_nbody_py, m)?)?;
    Ok(())
}