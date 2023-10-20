use nbodysolver::n_body_core;
use ndarray::prelude::*;
use numpy::{
    IntoPyArray, PyArray1, PyArray2, PyArray3, PyArrayLike1, PyArrayLike2, TypeMustMatch,
};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "all_planet_acc_nbody", signature=(r_list, m_list, g=6.6743e-11))]
pub fn all_planet_acc_nbody_py<'py>(
    py: Python<'py>,
    r_list: PyArrayLike2<'py, f64, TypeMustMatch>,
    m_list: PyArrayLike1<'py, f64, TypeMustMatch>,
    g: f64,
) -> &'py PyArray2<f64> {
    let acc_list: Array2<f64> =
        n_body_core::all_planet_acc_nbody(r_list.as_array(), m_list.as_array(), g);
    acc_list.into_pyarray(py)
}

#[pyfunction]
#[pyo3(name = "total_energy_nbody", signature=(r_list, v_list, m_list, g=6.6743e-11))]
pub fn total_energy_nbody_py<'py>(
    r_list: PyArrayLike2<'py, f64, TypeMustMatch>,
    v_list: PyArrayLike2<'py, f64, TypeMustMatch>,
    m_list: PyArrayLike1<'py, f64, TypeMustMatch>,
    g: f64,
) -> (f64, f64, f64) {
    let total_energy: (f64, f64, f64) =
        n_body_core::total_energy_nbody(r_list.as_array(), v_list.as_array(), m_list.as_array(), g);
    total_energy
}

#[pyfunction]
#[pyo3(name = "simulate_nbody_and_process", signature=(r_list, v_list, m_list, dt, max_time, g=6.6743e-11))]
pub fn simulate_nbody_and_process_py<'py>(
    py: Python<'py>,
    r_list: PyArrayLike2<'py, f64, TypeMustMatch>,
    v_list: PyArrayLike2<'py, f64, TypeMustMatch>,
    m_list: PyArrayLike1<'py, f64, TypeMustMatch>,
    dt: f64,
    max_time: f64,
    g: f64,
) -> (
    &'py PyArray1<f64>,
    &'py PyArray3<f64>,
    &'py PyArray3<f64>,
    &'py PyArray1<f64>,
    &'py PyArray1<f64>,
    &'py PyArray1<f64>,
) {
    let (all_t, all_r, all_v, all_ke, all_pe, all_te) = n_body_core::simulate_nbody(
        r_list.as_array(),
        v_list.as_array(),
        m_list.as_array(),
        dt,
        max_time,
        g,
    );
    let proc_r = n_body_core::process_data_nbody(all_r).into_pyarray(py);
    let proc_v = n_body_core::process_data_nbody(all_v).into_pyarray(py);
    (
        all_t.into_pyarray(py),
        proc_r,
        proc_v,
        all_ke.into_pyarray(py),
        all_pe.into_pyarray(py),
        all_te.into_pyarray(py),
    )
}
