use std::collections::HashMap;
use pyo3::prelude::*;
use nbodysolver::lorenz;

#[pyfunction]
#[pyo3(name = "solve_lorenz", signature = (coords, max_time, dt, tau=10.0, rho=28.0, beta=8.0/3.0))]
pub fn solve_lorenz_py(coords: (f64, f64, f64), max_time: f64, dt: f64, tau:f64, rho: f64, beta: f64) -> HashMap<String, Vec<f64>> {
    lorenz::solve_lorenz(coords, max_time, dt, tau, rho, beta)
}