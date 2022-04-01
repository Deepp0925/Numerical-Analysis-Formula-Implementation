/// this function will evaluate the hermite polynomial at given points x
/// # Arguments
/// * `x` - the point at which to evaluate the polynomial
/// * `x_points` - the points at which the polynomial is evaluated
/// * `y_points` - the points at which the polynomial is evaluated
/// * `y_prime_points` - the derivative of the polynomial at the points `x`
/// # Return
/// the value of the hermite polynomial at `x`
pub fn hermite(x: f64, x_points: &Vec<f64>, y_points: &Vec<f64>, y_prime_points: &Vec<f64>) -> f64 {
    let (hdd, z) = hdiv_diff(x_points, y_points, y_prime_points);
    let n = z.len() / 2;
    let mut hp = hdd[0][0];
    for i in 1..n * 2 {
        let mut d = 1.0;
        for j in 0..i {
            d *= x - z[j];
        }

        hp += d * hdd[i][i];
    }

    println!("{:#?}", hdd);

    hp
}

fn hdiv_diff(
    x_points: &Vec<f64>,
    y_points: &Vec<f64>,
    y_prime_points: &Vec<f64>,
) -> (Vec<Vec<f64>>, Vec<f64>) {
    let n = x_points.len();
    let h_len = n * 2;

    let mut z = vec![0.0; h_len];
    let mut q = vec![vec![-0.0; h_len]; h_len];

    for i in 0..n {
        let ind = i * 2;
        let nex_ind = ind + 1;
        z[ind] = x_points[i];
        z[nex_ind] = x_points[i];
        q[ind][0] = y_points[i];
        q[nex_ind][0] = y_points[i];
        q[nex_ind][1] = y_prime_points[i];
        if i != 0 {
            let prev_ind = ind - 1;
            q[ind][1] = (q[ind][0] - q[prev_ind][0]) / (z[ind] - z[prev_ind]);
        }
    }

    for i in 2..h_len {
        for j in 2..i + 1 {
            q[i][j] = (q[i][j - 1] - q[i - 1][j - 1]) / (z[i] - z[i - j]);
        }
    }

    return (q, z);
}
