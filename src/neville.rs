use meval::eval_str;

/// this function will calculate the neville's interpolation
/// # Arguments
/// * `x` - the x value
/// * `points` - the points to interpolate
/// # Returns
/// the interpolated value
pub fn neville_w_func(func: &str, points: &[f64], x: f64) -> Vec<Vec<f64>> {
    let mut y_points: Vec<f64> = vec![];

    // now those points at the func
    for i in 0..points.len() {
        let ans = eval_str(func.replace("x", points[i].to_string().as_str())).unwrap();
        y_points.push(ans);
    }

    // now we need to calculate the neville's interpolation
    return neville(points, &y_points, x);
}

pub fn neville(x_points: &[f64], y_points: &[f64], x: f64) -> Vec<Vec<f64>> {
    if x_points.len() != y_points.len() {
        panic!("x_points and y_points must be the same length");
    }
    let mut calc_points = Vec::new();
    // initialize the points
    for _ in 0..x_points.len() {
        calc_points.push(vec![]);
    }

    // now those points at the func
    for i in 0..x_points.len() {
        calc_points[i].push(y_points[i]);
    }

    // now the neville's interpolation
    for i in 1..x_points.len() {
        for j in 1..i + 1 {
            let res = ((x - x_points[i - j]) * (calc_points[i][j - 1])
                - (x - x_points[i]) * (calc_points[i - 1][j - 1]))
                / (x_points[i] - x_points[i - j]);
            calc_points[i].insert(j, res)
        }
    }

    return calc_points;
}
