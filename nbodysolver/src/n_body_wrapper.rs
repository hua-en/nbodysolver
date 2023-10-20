use crate::n_body_core::{process_data_nbody, simulate_nbody};
use ndarray::prelude::*;

pub struct NBodySystem {
    pub r_list: Array2<f64>,
    pub v_list: Array2<f64>,
    pub m_list: Array1<f64>,
}

pub struct NBodyResults {
    pub all_time: Array1<f64>,
    pub all_r: Array3<f64>,
    pub all_v: Array3<f64>,
    pub all_ke: Array1<f64>,
    pub all_pe: Array1<f64>,
    pub all_te: Array1<f64>,
}

pub fn simulate_system(dataset: NBodySystem, dt: f64, max_time: f64, g: f64) -> NBodyResults {
    let (all_time, all_r, all_v, all_ke, all_pe, all_te) = simulate_nbody(
        dataset.r_list,
        dataset.v_list,
        dataset.m_list,
        dt,
        max_time,
        g,
    );
    NBodyResults {
        all_time: Array::from_vec(all_time),
        all_r: process_data_nbody(all_r),
        all_v: process_data_nbody(all_v),
        all_ke: Array::from_vec(all_ke),
        all_pe: Array::from_vec(all_pe),
        all_te: Array::from_vec(all_te),
    }
}
