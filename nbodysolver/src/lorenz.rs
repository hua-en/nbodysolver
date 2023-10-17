use num::{traits::NumAssign, Float, FromPrimitive};

fn dxdt<T: Float>(x: T, y: T, tau: T) -> T {
    tau * (y - x)
}

fn dydt<T: Float>(x: T, y: T, z: T, rho: T) -> T {
    x * (rho - z) - y
}

fn dzdt<T: Float>(x: T, y: T, z: T, beta: T) -> T {
    x * y - beta * z
}

pub fn solve_lorenz<T: Float + NumAssign + FromPrimitive>(
    coords: (T, T, T),
    max_time: T,
    dt: T,
    tau: T,
    rho: T,
    beta: T,
) -> (Vec<T>, Vec<T>, Vec<T>, Vec<T>) {
    let mut x = coords.0;
    let mut y = coords.1;
    let mut z = coords.2;
    let mut t = T::from_f64(0.0).unwrap();

    let iteration_cnt = (max_time / dt).floor().to_usize().unwrap();

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

    (x_lst, y_lst, z_lst, t_lst)
}
