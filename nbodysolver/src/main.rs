use std::{fs::{File, self}, io::Write};
use nbodysolver::n_body_core::all_planet_acc_nbody;
use ndarray::prelude::*;
use serde::{Serialize, Deserialize};

fn main() -> std::io::Result<()> {
    let r_list = array![
        [0.0, 0.0, 0.0],
        [149.597e9, 0.0, 0.0],
        [149.981e9, 0.0, 0.0]
    ];
    let m_list = array![1.989e30, 5.972e24, 7.3476e22];
    let results = all_planet_acc_nbody(r_list, m_list, 6.6743e-11);
    let serialised = serde_json::to_string(&results).unwrap();
    fs::write("/tmp/foo.json", serialised).unwrap();
    println!("hello world");
    Ok(())
}
