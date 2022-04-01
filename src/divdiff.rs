use crate::factorial::factorial;

/// calculate the co-efficient of the a
fn coeff_forward(a: f64, n: i64) -> f64 {
    let mut tmp = a;
    for i in 1..n {
        tmp *= a - (i as f64);
    }
    tmp
}

/// Evaluates the points  using  newtons forward difference method.
/// # Arguments
/// * `x_points` - The x values of the points.
/// * `y_points` - The y values of the points.
/// * `x` - The x value to evaluate at.
/// # Returns
/// The y value of the point at x.
pub fn newton_forward_diff(
    x_points: &Vec<f64>,
    y_points: &Vec<f64>,
    x: f64,
    print_table: bool,
) -> f64 {
    if x_points.len() != y_points.len() {
        panic!("x_points and y_points must be the same length");
    }

    let num_points = x_points.len();
    let mut table = Vec::<Vec<f64>>::new();
    // place all y values in the first column
    for y in y_points {
        table.push(vec![*y]);
    }

    for i in 1..num_points {
        for j in 0..num_points - i {
            let res = table[j + 1][i - 1] - table[j][i - 1];
            table[j].insert(i, res);
        }
    }

    if print_table {
        for i in 0..num_points {
            print!("{} \t", x_points[i]);
            for j in 0..num_points - i {
                print!("{} \t", table[i][j]);
            }
            println!();
        }
    }

    let mut sum = table[0][0];
    let a = (x - x_points[0]) / (x_points[1] - x_points[0]);
    for i in 1..num_points {
        sum = sum + (coeff_forward(a, i as i64) * table[0][i] / factorial(i as i64) as f64);
    }

    return sum;
}

fn coeff_backward(a: f64, n: i64) -> f64 {
    let mut tmp = a;
    for i in 1..n {
        tmp *= a + (i as f64);
    }
    tmp
}

/// Evaluates the points  using  newtons backward difference method.
/// # Arguments
/// * `x_points` - The x values of the points.
/// * `y_points` - The y values of the points.
/// * `x` - The x value to evaluate at.
/// # Returns
/// The y value of the point at x.
pub fn newton_backward_diff(
    x_points: &Vec<f64>,
    y_points: &Vec<f64>,
    x: f64,
    print_table: bool,
) -> f64 {
    if x_points.len() != y_points.len() {
        panic!("x_points and y_points must be the same length");
    }

    let num_points = x_points.len();
    let mut table = Vec::<Vec<f64>>::new();
    // place all y values in the first column
    for y in y_points {
        table.push(vec![*y]);
    }

    for i in 1..num_points {
        let mut j = num_points - 1;
        while j >= i {
            let res = table[j][i - 1] - table[j - 1][i - 1];
            table[j].insert(i, res);
            j -= 1;
        }
    }

    if print_table {
        for i in 0..num_points {
            print!("{} \t", x_points[i]);
            for j in 0..i + 1 {
                print!("{} \t", table[i][j]);
            }
            println!();
        }
    }

    let mut sum = table[num_points - 1][0];
    let a = (x - x_points[num_points - 1]) / (x_points[1] - x_points[0]);
    for i in 1..num_points {
        sum = sum
            + (coeff_backward(a, i as i64) * table[num_points - 1][i] / factorial(i as i64) as f64);
    }

    return sum;
}
