mod lorenz_py;
mod n_body_py;
mod n_body_py_faster;

use pyo3::prelude::*;

use crate::lorenz_py::solve_lorenz_py;
use crate::n_body_py::{
    all_planet_acc_nbody_py, simulate_nbody_and_process_py, total_energy_nbody_py,
};
use crate::n_body_py_faster::{
    all_planet_acc_nbody_py_faster, simulate_nbody_and_process_py_faster, total_energy_nbody_py_faster,
};

/// A Python module implemented in Rust.
#[pymodule]
fn nbodysolver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_lorenz_py, m)?)?;
    m.add_function(wrap_pyfunction!(all_planet_acc_nbody_py, m)?)?;
    m.add_function(wrap_pyfunction!(total_energy_nbody_py, m)?)?;
    m.add_function(wrap_pyfunction!(simulate_nbody_and_process_py, m)?)?;
    m.add_function(wrap_pyfunction!(all_planet_acc_nbody_py_faster, m)?)?;
    m.add_function(wrap_pyfunction!(total_energy_nbody_py_faster, m)?)?;
    m.add_function(wrap_pyfunction!(simulate_nbody_and_process_py_faster, m)?)?;
    Ok(())
}
