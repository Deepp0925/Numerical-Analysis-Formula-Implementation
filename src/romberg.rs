/// the function evaluates the romberg integration
/// # Arguments
/// * `a` - the lower bound of the integral
/// * `b` - the upper bound of the integral
/// * `f` - the function to integrate
/// * `n` - the number of steps to take
/// # Returns
/// the value of the integral and also prints the table of values
pub fn romberg(a: f64, b: f64, f: &dyn Fn(f64) -> f64, n: u64) -> f64 {
    let mut h = (b - a);
    let mut r: Vec<Vec<f64>> = Vec::new();

    // insert the first row, R1,1
    r.push(vec![(h / 2.0) * (f(a) + f(b))]);

    for i in 1..n + 1 {
        let i = i as usize;
        h = h / 2.0;
        let mut sum = 0.0;

        for k in (1..(2_u64.pow(i as u32))).step_by(2) {
            sum = sum + f(a + k as f64 * h);
        }

        // r[i,0]
        let mut rowi = vec![0.5 * r[i - 1][0] + sum * h];
        for j in 1..(i + 1) {
            let j = j as usize;
            let rij = rowi[j - 1] + (rowi[j - 1] - r[i - 1][j - 1]) / (4.0f64.powi(j as i32) - 1.0);
            rowi.push(rij);
        }
        println!("{:?}", rowi);
        r.push(rowi);
    }

    return 0.0;
}
