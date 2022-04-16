use std::f64::consts::PI;
/// beware this function is currently not working accurately, please use gaussian.java instead
/// this function is used to calculate the gaussian quadrature over a given interval
/// # Arguments
/// * `a` - the lower bound of the interval
/// * `b` - the upper bound of the interval
/// * `f` - the function to integrate
/// * `n` - the number of steps to take
/// # Returns
/// the value of the integral
pub fn gaussian_quadrature(a: f64, b: f64, f: &dyn Fn(f64) -> f64, n: u64) -> f64 {
    let mut lroots = vec![-0.0; n as usize];
    let mut weights = vec![-0.0; n as usize];
    let mut lcoef = vec![vec![-0.0; n as usize + 1]; n as usize + 1];

    // calculates the legendre polynomial coefficients
    lcoef[0][0] = 1.0;
    lcoef[1][1] = 1.0;

    for i in 2..n + 1 {
        let i = i as usize;
        lcoef[i][0] = -((i - 1) as f64) * lcoef[i - 2][0] / i as f64;

        for j in 1..i + 1 {
            lcoef[i][j] = ((2.0 * (i as f64) - 1.0) * lcoef[i - 1][j - 1]
                - (i - 1) as f64 * lcoef[i - 2][j])
                / (i as f64);
        }
    }

    let lege_eval = |m: i64, x: f64| -> f64 {
        let mut s = lcoef[m as usize][m as usize];
        let mut i = m;
        while i > 0 {
            s = s * x + lcoef[m as usize][i as usize - 1];
            i -= 1;
        }

        s
    };

    let lege_diff = |m: i64, x: f64| -> f64 {
        m as f64 * (x * lege_eval(m, x) - lege_eval(m - 1, x)) / (x * x - 1.0)
    };

    // legendre polynomial roots
    let mut lege_roots = || {
        let mut x = 0.0;
        let mut x1 = 0.0;
        for i in 1..n + 1 {
            let i = i as usize;
            x = (PI * (i as f64 - 0.25)) / (n as f64 + 0.5).cos();
            loop {
                x1 = x;
                x -= (lege_eval(n as i64, x) / lege_diff(n as i64, x));

                if x == x1 {
                    break;
                }
            }

            lroots[i - 1] = x;
            x1 = lege_diff(n as i64, x);
            weights[i - 1] = 2.0 / ((1.0 - x * x) * x1 * x1);
        }
    };

    lege_roots();

    let c1 = (b - a) / 2.0;
    let c2 = (b + a) / 2.0;
    let mut sum = 0.0;

    for i in 0..n {
        sum += weights[i as usize] * f(c1 * lroots[i as usize] + c2);
    }

    return c1 * sum;
}
