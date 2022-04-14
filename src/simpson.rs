/// the function evaluates the simpson rule for the integral of a function
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the function to integrate
/// # Returns
/// the value of the integral
pub fn simpson(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) / 2.0;
    let sum = f(a) + 4.0 * f(a + h) + f(b);
    sum * (h / 3.0)
}

/// this function evaluates the composite simpson rule for the integral of a function
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the function to integrate
/// * `n` - the number of steps to take
/// # Returns
/// the value of the integral
pub fn composite_simpson(a: f64, b: f64, f: &dyn Fn(f64) -> f64, n: u64) -> f64 {
    let h = (b - a) / n as f64;
    let x0 = f(a) + f(b);

    let mut x1 = 0.0;
    let mut x2 = 0.0;

    for i in 1..n {
        let x = a + i as f64 * h;
        if i % 2 == 1 {
            x1 += f(x);
        } else {
            x2 += f(x);
        }
    }
    (x0 + 2.0 * x2 + 4.0 * x1) * (h / 3.0)
}

/// this function evaluates the composite simpson error term
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the fourth deriavtive of the function to integrate(f''''(x))
/// * `n` - the number of steps to take
/// # Returns
/// the value of the error term
pub fn composite_simpson_err(a: f64, b: f64, f: &dyn Fn(f64) -> f64, n: u64) -> f64 {
    let h = (b - a) / n as f64;
    (b - a / 180.0) * h.powi(4) * f(b)
}

/// this function evaluates the simpson error term
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the fourth deriavtive of the function to integrate(f''''(x))
/// # Returns
/// the value of the error term
pub fn simpson_err(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) / 2.0;
    (h.powi(5) / 90.0) * f(b)
}
