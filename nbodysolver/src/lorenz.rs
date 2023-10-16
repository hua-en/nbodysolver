use std::collections::HashMap;

fn dxdt(x:f64, y:f64, tau:f64) -> f64 {
    tau * (y - x)
}

fn dydt(x: f64, y:f64, z:f64, rho:f64) -> f64 {
    x * (rho - z) - y
}

fn dzdt(x: f64, y:f64, z:f64, beta:f64) -> f64 {
    x * y - beta * z
}

pub fn solve_lorenz(coords: (f64, f64, f64), max_time: f64, dt: f64, tau:f64, rho: f64, beta: f64) -> HashMap<String, Vec<f64>> {
    let mut x = coords.0;
    let mut y = coords.1;
    let mut z = coords.2;
    let mut t = 0.0;

    let iteration_cnt = (max_time / dt).floor() as usize;

    let mut x_lst = Vec::with_capacity(iteration_cnt);
    let mut y_lst = Vec::with_capacity(iteration_cnt);
    let mut z_lst = Vec::with_capacity(iteration_cnt);
    let mut t_lst = Vec::with_capacity(iteration_cnt);

    // Euler Method
    for _ in 0..iteration_cnt {
        x_lst.push(x);
        y_lst.push(y);
        z_lst.push(z);
        t_lst.push(t);

        x += dxdt(x, y, tau) * dt;
        y += dydt(x, y, z, rho) * dt;
        z += dzdt(x, y, z, beta) * dt;
        t += dt;
    }
    
    // Create return map
    let mut ret_map = HashMap::new();
    ret_map.insert("x".to_string(), x_lst);
    ret_map.insert("y".to_string(), y_lst);
    ret_map.insert("z".to_string(), z_lst);
    ret_map.insert("t".to_string(), t_lst);

    ret_map
}