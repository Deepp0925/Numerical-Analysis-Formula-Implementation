/// This functions evaluates the composite trapazoidal rule for the integral of a function.
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the function to integrate
/// * `n` - the number of steps to take
/// # Returns
/// the value of the integral
pub fn composite_trapazoidal(a: f64, b: f64, f: &dyn Fn(f64) -> f64, n: u64) -> f64 {
    let h = (b - a) / n as f64;
    // adds the first and last term
    let mut sum = f(a) + f(b);

    // does sumation
    for i in 1..n {
        sum += (2.0 * f(a + i as f64 * h));
    }
    // returns the value with (h/2)*sum
    sum * (h / 2.0)
}

/// this function evaluates the composite trapazoidal error term
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the second deriavtive of the function to integrate(f''(x))
/// * `n` - the number of steps to take
/// # Returns
/// the value of the error term
pub fn composite_trapazoidal_err(a: f64, b: f64, f: &dyn Fn(f64) -> f64, n: u64) -> f64 {
    let h = (b - a) / n as f64;
    ((b - a) / 12.0) * h.powi(2) * f(b)
}

/// this fnction evaluates the trapezoidal rule for the integral of a function
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the function to integrate
/// # Returns
/// the value of the integral
pub fn trapazoidal(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) / 2.0;
    let mut sum = f(a) + f(b);
    sum * h
}

/// this function evaluate the trapezoidal rule error term
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the second deriavtive of the function to integrate(f''(x))
/// * `n` - the number of steps to take
/// # Returns
/// the value of the error term
pub fn trapazoidal_err(a: f64, b: f64, f: &dyn Fn(f64) -> f64) -> f64 {
    let h = (b - a) as f64;
    (h.powi(3) / 12.0) * f(b)
}
