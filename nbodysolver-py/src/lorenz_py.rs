use nbodysolver::lorenz;
use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "solve_lorenz", signature = (coords, max_time, dt, tau=10.0, rho=28.0, beta=8.0/3.0))]
pub fn solve_lorenz_py<'py>(
    py: Python<'py>,
    coords: (f64, f64, f64),
    max_time: f64,
    dt: f64,
    tau: f64,
    rho: f64,
    beta: f64,
) -> (
    &'py PyArray1<f64>,
    &'py PyArray1<f64>,
    &'py PyArray1<f64>,
    &'py PyArray1<f64>,
) {
    let (x_lst, y_lst, z_lst, t_lst) = lorenz::solve_lorenz(coords, max_time, dt, tau, rho, beta);
    (
        x_lst.into_pyarray(py),
        y_lst.into_pyarray(py),
        z_lst.into_pyarray(py),
        t_lst.into_pyarray(py),
    )
}
