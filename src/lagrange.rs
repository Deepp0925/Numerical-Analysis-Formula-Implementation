use meval::eval_str;

/// this function implemeents the Lagrange interpolation
/// for a set of points
/// # Arguments
/// * `x_points` - the x values of the points
/// * `y_points` - the y values of the points
/// * `x` - the x value to interpolate at
/// # Returns
/// the interpolated y value
pub fn lagrange(x_points: &[f64], y_points: &[f64], x: f64) -> f64 {
    let mut result = 0.0;
    for i in 0..x_points.len() {
        let mut l = 1.0;
        for j in 0..x_points.len() {
            if i != j {
                l *= (x - x_points[j]) / (x_points[i] - x_points[j]);
            }
        }
        result += l * y_points[i];
    }
    return result;
}

/// this function implemeents the Lagrange interpolation
/// for a set of points
/// # Arguments
/// * `func` - the function to interpolate
/// * `x_points` - the x values of the points
/// * `x` - the x value to interpolate at
/// # Returns
/// the interpolated y value
pub fn lagrange_w_func(func: &str, x_points: &[f64], x: f64) -> f64 {
    let mut y_points: Vec<f64> = vec![];

    // now eval those points at the func
    for i in 0..x_points.len() {
        let ans = eval_str(func.replace("x", x_points[i].to_string().as_str())).unwrap();
        y_points.push(ans);
    }

    return lagrange(x_points, &y_points, x);
}
