use std::{fs::{File, self}, io::Write};
use nbodysolver::{n_body_core::all_planet_acc_nbody, NBodySystem, simulate_system};
use ndarray::prelude::*;
use serde::{Serialize, Deserialize};
use plotters::prelude::*;

fn main() -> std::io::Result<()> {
    let r_list = array![
        [0.0, 0.0, 0.0],
        [149.597e9, 0.0, 0.0],
        [149.981e9, 0.0, 0.0]
    ];
    let m_list = array![1.989e30, 5.972e24, 7.3476e22];
    let acc_results = all_planet_acc_nbody(r_list, m_list, 6.6743e-11);
    let serialised = serde_json::to_string(&acc_results).unwrap();
    fs::write("/tmp/foo.json", serialised).unwrap();
    println!("hello world");
    
    let sun_earth_moon_system = NBodySystem {
        r_list: array![
            [0.0, 0.0, 0.0],          // Position of first body, Sun
            [149.597e9, 0.0, 0.0],    // Position of second body, Earth
            [149.981e9, 0.0, 0.0]     // Position of third body, Moon
        ],
        v_list: array![
            [0.0, 0.0, 0.0],          // Velocity of first body, Sun
            [0.0, 29800.0, 0.0],      // Velocity of second body, Earth
            [0.0, 30800.0, 0.0]       // Velocity of third body, Moon
        ],
        m_list: array![
            1.989e30,                 // Mass of first body, Sun
            5.972e24,                 // Mass of second body, Earth
            7.3476e22                 // Mass of third body, Moon
        ]
    };
    let sem_results = simulate_system(sun_earth_moon_system, 1000.,31536000., 6.67e-11);

    Ok(())
}
