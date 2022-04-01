/// this will extrapolate at a given point x
/// # Arguments
/// * `x` - the point at which to extrapolate
/// * `f` - the function to extrapolate -- it will be called iteratively with floats as arguments
/// * `h` - the step size
/// * `n` - the number of steps to take/ the level of extrapolation
/// # Returns
/// the extrapolated value
pub fn r_extrapolate(x: f64, f: &dyn Fn(f64) -> f64, mut h: f64, n: i64) -> f64 {
    let len = (n) as usize;
    let mut d = vec![vec![-0.0; len]; len];

    for i in 0..len {
        d[i][0] = 0.5 * (f(x + h) - f(x - h)) / h;

        let mut powerOf4: f64 = 1.0;
        for j in 1..i + 1 {
            powerOf4 = 4.0 * powerOf4;
            d[i][j] = d[i][j - 1] + (d[i][j - 1] - d[i - 1][j - 1]) / (powerOf4 - 1.0);
        }

        h = 0.5 * h;
    }

    /// this is just for printing purposes
    for r in d {
        for i in r {
            if (i != -0.0) {
                println!("{}", i);
            }
        }
        println!();
    }
    return 0.0;
}
