use std::collections::HashMap;
use std::iter::zip;
use pyo3::prelude::*;
use ndarray::prelude::*;
use ndarray::Array;
use numpy::{IntoPyArray, PyArray1};
use ndarray_linalg::norm;

fn all_planet_acc_nbody(r_list: &[ArrayView1<f64>], m_list: &[f64], G:f64) -> Vec<Array1<f64>> {
    let planet_cnt = r_list.len();
    let mut acc_list = Vec::with_capacity(planet_cnt);

    for i in 0..planet_cnt {
        let mut i_acc = array![0.0, 0.0, 0.0];
        for j in 0..planet_cnt {
            if i != j {
                let rij = &r_list[i] - &r_list[j];
                i_acc += &((-G * m_list[j] * (&rij)) / (norm::Norm::norm_l2(&rij).powi(3)))
            }
        }
        acc_list.push(i_acc)
    }
    acc_list
}

#[pyfunction]
pub fn all_planet_acc_nbody_py(r_list: Vec<Vec<f64>>, m_list: Vec<f64>, G:f64) -> Vec<Vec<f64>> {
    let r_list: Vec<Array1<f64>> = r_list.iter().map(|v| Array::from_vec(v.clone())).collect();
    let r_list_view: Vec<ArrayView1<f64>> = r_list.iter().map(|v| v.view()).collect();
    let acc_list: Vec<Vec<f64>> = all_planet_acc_nbody(&r_list_view, &m_list, G).iter().map(|a| a.to_vec()).collect();
    acc_list
}

fn total_energy_nbody(r_list: &[ArrayView1<f64>], V_list: &[ArrayView1<f64>], m_list: &[f64], G: f64) -> (f64, f64, f64) {
    let mut kinetic_energy = 0.0;
    let mut potential_energy = 0.0;

    // Total Kinetic Energy
    for (v, m) in zip(V_list, m_list) {
        kinetic_energy += 0.5 * *m * (norm::Norm::norm_l2(v).powi(2));
    }

    // Total Potential Energy
    let planet_cnt = r_list.len();
    for i in 0..(planet_cnt-1) {
        for j in (i+1)..planet_cnt {
            potential_energy += (-G * m_list[i] * m_list[j]) / norm::Norm::norm_l2(&(&r_list[i] - &r_list[j]));
        }
    }

    // Return KE, PE, TE
    (kinetic_energy, potential_energy, kinetic_energy + potential_energy)
}

#[pyfunction]
pub fn total_energy_nbody_py(r_list: Vec<Vec<f64>>, V_list: Vec<Vec<f64>>, m_list: Vec<f64>, G: f64) -> (f64, f64, f64) {
    let r_list: Vec<Array1<f64>> = r_list.iter().map(|v| Array::from_vec(v.clone())).collect();
    let r_list_view: Vec<ArrayView1<f64>> = r_list.iter().map(|v| v.view()).collect();
    let V_list: Vec<Array1<f64>> = V_list.iter().map(|v| Array::from_vec(v.clone())).collect();
    let v_list_view: Vec<ArrayView1<f64>> = V_list.iter().map(|v| v.view()).collect();
    let total_energy: (f64, f64, f64) = total_energy_nbody(&r_list_view, &v_list_view, &m_list, G);
    total_energy
}

pub fn simulate_nbody() {
    println!("Hello World");
}

fn explicit_euler() {

}

fn semi_implicit_euler() {

}

fn leapfrog() {

}

fn process_pos_data_nbody() {

}

fn simulate_nbody_and_process_py() {

}

// Example of a function accepting python lists and returning a numpy array.
#[pyfunction]
pub fn list_sum<'py>(py: Python<'py>, a: Vec<i64>, b: Vec<i64>) -> &'py PyArray1<i64> {
    let a = Array::from_vec(a);
    let b = Array::from_vec(b);
    let c = a + b;
    c.into_pyarray(py)
}
