mod lorenz;
mod n_body;

use pyo3::prelude::*;

use crate::lorenz::solve_lorenz;
use crate::n_body::{
    all_planet_acc_nbody_py, simulate_nbody_and_process_py, total_energy_nbody_py,
};

/// A Python module implemented in Rust.
#[pymodule]
fn nbodysolver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_lorenz, m)?)?;
    m.add_function(wrap_pyfunction!(all_planet_acc_nbody_py, m)?)?;
    m.add_function(wrap_pyfunction!(total_energy_nbody_py, m)?)?;
    m.add_function(wrap_pyfunction!(simulate_nbody_and_process_py, m)?)?;
    Ok(())
}
