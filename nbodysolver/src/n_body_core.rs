use ndarray::{prelude::*, stack};
use ndarray::{Array, Data};
use ndarray_linalg::norm;
use std::iter::zip;
use std::mem;
use std::ops::AddAssign;

pub fn all_planet_acc_nbody<S: Data<Elem = f64>>(
    r_list: ArrayBase<S, Ix2>,
    m_list: ArrayBase<S, Ix1>,
    g: f64,
) -> Array2<f64> {
    let planet_cnt = r_list.nrows();
    let mut acc_list = Array::zeros(r_list.raw_dim());

    for i in 0..planet_cnt {
        for j in 0..planet_cnt {
            if i != j {
                let rij = &r_list.row(i) - &r_list.row(j);
                acc_list
                    .row_mut(i)
                    .add_assign(&((-g * m_list[j] * (&rij)) / (norm::Norm::norm_l2(&rij).powi(3))))
            }
        }
    }
    acc_list
}

pub fn total_energy_nbody<S: Data<Elem = f64>>(
    r_list: ArrayBase<S, Ix2>,
    v_list: ArrayBase<S, Ix2>,
    m_list: ArrayBase<S, Ix1>,
    g: f64,
) -> (f64, f64, f64) {
    let mut kinetic_energy = 0.0;
    let mut potential_energy = 0.0;

    // Total Kinetic Energy
    for (v, m) in zip(v_list.outer_iter(), m_list.iter()) {
        kinetic_energy += 0.5 * *m * (norm::Norm::norm_l2(&v).powi(2));
    }

    // Total Potential Energy
    let planet_cnt = r_list.nrows();
    for i in 0..(planet_cnt - 1) {
        for j in (i + 1)..planet_cnt {
            potential_energy += (-g * m_list[i] * m_list[j])
                / norm::Norm::norm_l2(&(&r_list.row(i) - &r_list.row(j)));
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
    Vec<Array2<f64>>,
    Vec<Array2<f64>>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
);

pub fn simulate_nbody<S: Data<Elem = f64>>(
    r_list_init: ArrayBase<S, Ix2>,
    v_list_init: ArrayBase<S, Ix2>,
    m_list: ArrayBase<S, Ix1>,
    dt: f64,
    max_time: f64,
    g: f64,
) -> NBodyResults {
    // Create a copy of r_list and V_list
    let mut r_list: Array2<f64> = r_list_init.to_owned();
    let mut v_list: Array2<f64> = v_list_init.to_owned();
    let mut a_list: Array2<f64> = all_planet_acc_nbody(r_list.view(), m_list.view(), g);

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
    let (_, _, init_te) = total_energy_nbody(r_list.view(), v_list.view(), m_list.view(), g);
    let abs_init_te = init_te.abs();

    for time in all_time {
        // Calculate current energy values
        let (ke, pe, te) = total_energy_nbody(r_list.view(), v_list.view(), m_list.view(), g);

        // Check if TE has changed too much from the initial total energy
        // If it has, stop the simulation early
        let abs_te = te.abs();
        if !(abs_init_te * 0.5 < abs_te && abs_te < abs_init_te * 1.5) {
            break;
        }

        // Calculate new position and velocity
        let (new_r_list, new_v_list, new_a_list) = leapfrog(
            r_list.view(),
            v_list.view(),
            a_list.view(),
            m_list.view(),
            dt,
            g,
        );

        // Set position and velocity to next timestep
        let old_r_list = mem::replace(&mut r_list, new_r_list);
        let old_v_list = mem::replace(&mut v_list, new_v_list);
        a_list = new_a_list;

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
    r_list: ArrayBase<S, Ix2>,
    v_list: ArrayBase<S, Ix2>,
    a_list: ArrayBase<S, Ix2>,
    m_list: ArrayBase<S, Ix1>,
    dt: f64,
    g: f64,
) -> (Array2<f64>, Array2<f64>, Array2<f64>) {
    // Find the new positions of the objects
    let new_r_list = &r_list + &v_list * dt + 0.5 * &a_list * (dt.powi(2));

    // Find the new acceleration of the objects
    let new_a_list = all_planet_acc_nbody(new_r_list.view(), m_list.view(), g);

    // Find the new velocity of the objects
    let new_v_list = &v_list + 0.5 * (&a_list + &new_a_list) * dt;

    (new_r_list, new_v_list, new_a_list)
}

pub fn process_data_nbody(pos_data: Vec<Array2<f64>>) -> Array3<f64> {
    let d1 = pos_data.len();
    let (d2, d3) = pos_data[0].dim();
    let flat: Vec<f64> = pos_data.into_iter().flatten().collect();
    Array3::from_shape_vec((d1, d2, d3), flat).unwrap()
}