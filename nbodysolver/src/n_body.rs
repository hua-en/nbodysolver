use itertools::izip;
use ndarray::{prelude::*, stack};
use ndarray::{Array, Data};
use ndarray_linalg::norm;
use std::iter::zip;
use std::mem;

pub fn all_planet_acc_nbody<S: Data<Elem = f64>>(
    r_list: &[ArrayBase<S, Ix1>],
    m_list: &[f64],
    g: f64,
) -> Vec<Array1<f64>> {
    let planet_cnt = r_list.len();
    let mut acc_list = Vec::with_capacity(planet_cnt);

    for i in 0..planet_cnt {
        let mut i_acc = array![0.0, 0.0, 0.0];
        for j in 0..planet_cnt {
            if i != j {
                let rij = &r_list[i] - &r_list[j];
                i_acc += &((-g * m_list[j] * (&rij)) / (norm::Norm::norm_l2(&rij).powi(3)))
            }
        }
        acc_list.push(i_acc)
    }
    acc_list
}

pub fn total_energy_nbody<S: Data<Elem = f64>>(
    r_list: &[ArrayBase<S, Ix1>],
    v_list: &[ArrayBase<S, Ix1>],
    m_list: &[f64],
    g: f64,
) -> (f64, f64, f64) {
    let mut kinetic_energy = 0.0;
    let mut potential_energy = 0.0;

    // Total Kinetic Energy
    for (v, m) in zip(v_list, m_list) {
        kinetic_energy += 0.5 * *m * (norm::Norm::norm_l2(v).powi(2));
    }

    // Total Potential Energy
    let planet_cnt = r_list.len();
    for i in 0..(planet_cnt - 1) {
        for j in (i + 1)..planet_cnt {
            potential_energy +=
                (-g * m_list[i] * m_list[j]) / norm::Norm::norm_l2(&(&r_list[i] - &r_list[j]));
        }
    }

    // Return KE, PE, TE
    (
        kinetic_energy,
        potential_energy,
        kinetic_energy + potential_energy,
    )
}

type NBodyResults = (
    Vec<f64>,
    Vec<Vec<Array1<f64>>>,
    Vec<Vec<Array1<f64>>>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
);

pub fn simulate_nbody<S: Data<Elem = f64>>(
    r_list_init: &[ArrayBase<S, Ix1>],
    v_list_init: &[ArrayBase<S, Ix1>],
    m_list: &[f64],
    dt: f64,
    max_time: f64,
    g: f64,
) -> NBodyResults {
    // Create a copy of r_list and V_list
    let mut r_list: Vec<Array1<f64>> = r_list_init.iter().map(|v| v.to_owned()).collect();
    let mut v_list: Vec<Array1<f64>> = v_list_init.iter().map(|v| v.to_owned()).collect();

    // Initialise all datasets
    let all_time = Array::range(0., max_time, dt);
    let timespan = all_time.len();

    let mut all_t = Vec::with_capacity(timespan);
    let mut all_r = Vec::with_capacity(timespan);
    let mut all_v = Vec::with_capacity(timespan);
    let mut all_te = Vec::with_capacity(timespan);
    let mut all_pe = Vec::with_capacity(timespan);
    let mut all_ke = Vec::with_capacity(timespan);

    // Calculate initial total energy
    let (_, _, init_te) = total_energy_nbody(&r_list, &v_list, m_list, g);
    let abs_init_te = init_te.abs();

    for time in all_time {
        // Calculate current energy values
        let (ke, pe, te) = total_energy_nbody(&r_list, &v_list, m_list, g);

        // Check if TE has changed too much from the initial total energy
        // If it has, stop the simulation early
        let abs_te = te.abs();
        if !(abs_init_te * 0.5 < abs_te && abs_te < abs_init_te * 1.5) {
            break;
        }

        // Calculate new position and velocity
        let (new_r_list, new_v_list) = leapfrog(&r_list, &v_list, m_list, dt, g);

        // Set position and velocity to next timestep
        let old_r_list = mem::replace(&mut r_list, new_r_list);
        let old_v_list = mem::replace(&mut v_list, new_v_list);

        // Record current position and time in dataset
        all_t.push(time);
        all_r.push(old_r_list);
        all_v.push(old_v_list);

        // Record current energy values in dataset
        all_ke.push(ke);
        all_pe.push(pe);
        all_te.push(te);
    }

    (all_t, all_r, all_v, all_ke, all_pe, all_te)
}

pub fn explicit_euler() {}

pub fn semi_implicit_euler() {}

pub fn leapfrog<S: Data<Elem = f64>>(
    r_list: &[ArrayBase<S, Ix1>],
    v_list: &[ArrayBase<S, Ix1>],
    m_list: &[f64],
    dt: f64,
    g: f64,
) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
    let planet_cnt = r_list.len();
    // Find the acceleration of the objects
    let a_list = all_planet_acc_nbody(r_list, m_list, g);

    // Find the new positions of the objects
    let mut new_r_list = Vec::with_capacity(planet_cnt);
    for (r, v, a) in izip!(r_list, v_list, &a_list) {
        let new_r = r + v * dt + 0.5 * a * (dt.powi(2));
        new_r_list.push(new_r)
    }

    // Find the new acceleration of the objects
    let new_a_list = all_planet_acc_nbody(&new_r_list, m_list, g);

    // Find the new velocity of the objects
    let mut new_v_list = Vec::with_capacity(planet_cnt);
    for (v, a, new_a) in izip!(v_list, &a_list, &new_a_list) {
        let new_v = v + 0.5 * (a + new_a) * dt;
        new_v_list.push(new_v);
    }

    (new_r_list, new_v_list)
}

pub fn process_data_nbody(pos_data: Vec<Vec<Array1<f64>>>) -> Array2<f64> {
    let rows = pos_data.len();
    let columns = pos_data[0].len() * 3;
    let flattened = pos_data.into_iter().flatten().flatten().collect();
    Array2::from_shape_vec((rows, columns), flattened).unwrap()
}