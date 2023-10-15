use std::collections::HashMap;
use std::iter::zip;
use std::mem;
use itertools::izip;
use itertools::multiunzip;
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

type NBodyResults = (Vec<f64>, Vec<Vec<Array1<f64>>>, Vec<Vec<Array1<f64>>>, Vec<f64>, Vec<f64>, Vec<f64>);

fn simulate_nbody(r_list_init: &Vec<ArrayView1<f64>>, V_list_init: &Vec<ArrayView1<f64>>, m_list: &[f64], dt: f64, max_time: f64, G: f64) -> NBodyResults {
    // Create a copy of r_list and V_list
    let mut r_list: Vec<Array1<f64>> = r_list_init.iter().map(|v| v.to_owned()).collect();
    let mut V_list: Vec<Array1<f64>> = V_list_init.iter().map(|v| v.to_owned()).collect();

    // Initialise all datasets
    let all_time = Array::range(0., max_time, dt);
    let timespan = all_time.len();
    
    let mut all_t = Vec::with_capacity(timespan);
    let mut all_r = Vec::with_capacity(timespan);
    let mut all_V = Vec::with_capacity(timespan);
    let mut all_TE = Vec::with_capacity(timespan);
    let mut all_PE = Vec::with_capacity(timespan);
    let mut all_KE = Vec::with_capacity(timespan);

    // Calculate initial total energy
    let r_list_view: Vec<ArrayView1<f64>> = r_list.iter().map(|v| v.view()).collect();
    let v_list_view: Vec<ArrayView1<f64>> = V_list.iter().map(|v| v.view()).collect();
    let (_, _, init_TE) = total_energy_nbody(&r_list_view, &v_list_view, m_list, G);
    let abs_init_TE = init_TE.abs();

    for time in all_time {
        // Create arrayviews for current r_list and V_list
        let r_list_view: Vec<ArrayView1<f64>> = r_list.iter().map(|v| v.view()).collect();
        let v_list_view: Vec<ArrayView1<f64>> = V_list.iter().map(|v| v.view()).collect();

        // Calculate current energy values
        let (KE, PE, TE) = total_energy_nbody(&r_list_view, &v_list_view, m_list, G);

        // Check if TE has changed too much from the initial total energy
        // If it has, stop the simulation early
        let abs_TE = TE.abs();
        if !(abs_init_TE * 0.5 < abs_TE && abs_TE < abs_init_TE * 1.5) {
            break;
        }

        // Calculate new position and velocity
        let (new_r_list, new_V_list) = leapfrog(&r_list_view, &v_list_view, m_list, dt, G);

        // Set position and velocity to next timestep
        let old_r_list = mem::replace(&mut r_list, new_r_list);
        let old_V_list = mem::replace(&mut V_list, new_V_list);

        // Record current position and time in dataset
        all_t.push(time);
        all_r.push(old_r_list);
        all_V.push(old_V_list);

        // Record current energy values in dataset        
        all_KE.push(KE);
        all_PE.push(PE);
        all_TE.push(TE);
    }

    (all_t, all_r, all_V, all_KE, all_PE, all_TE)
}

fn explicit_euler() {

}

fn semi_implicit_euler() {

}

fn leapfrog(r_list: &[ArrayView1<f64>], V_list: &[ArrayView1<f64>], m_list: &[f64], dt: f64, G: f64) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
    let planet_cnt = r_list.len();
    // Find the acceleration of the objects
    let a_list = all_planet_acc_nbody(r_list, m_list, G);
    
    // Find the new positions of the objects
    let mut new_r_list = Vec::with_capacity(planet_cnt);
    for (r, V, a) in izip!(r_list, V_list, &a_list) {
        let new_r = r + V * dt + 0.5 * a * (dt.powi(2));
        new_r_list.push(new_r)
    }

    let new_r_list_view: Vec<ArrayView1<f64>> = new_r_list.iter().map(|v| v.view()).collect();
    let new_a_list = all_planet_acc_nbody(&new_r_list_view, m_list, G);

    // Find the new velocity of the objects
    let mut new_V_list = Vec::with_capacity(planet_cnt);
    for (V, a, new_a) in izip!(V_list, &a_list, &new_a_list) {
        let new_V = V + 0.5 * (a + new_a) * dt;
        new_V_list.push(new_V);
    }

    (new_r_list, new_V_list)
}

fn process_pos_data_nbody(pos_data: Vec<Vec<Array1<f64>>>) {
    // Figure out a way to convert a list of lists of coordinates at various times to a list of lists of the separated coordinates
    // i.e [[(x10, y10, z10), (x20, y20, z20), ...], [(x11, y11, z11), ...]] => 
    // [[[x10, x11, ...], [y10, y11, ...], [z10, z11, ...]], [[x20, x21, ...], [y20, y21, ...], [z20, z21, ...]]]
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
