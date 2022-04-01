use meval::eval_str;
pub fn question3(x: f64) -> f64 {
    (x - 2.0).powi(2) - x.ln()
}

pub fn q3_secant(x_2: f64, x_1: f64, iterations: i8, print_iter: bool) -> f64 {
    let mut x_2 = x_2;
    let mut x_1 = x_1;

    let mut last_result: f64 = 0.0;

    let mut iter = 0;
    while iter < iterations {
        // calculates f(x-1)(x-2 - x-1)
        let numerator_1 = question3(x_1);
        let numerator_2 = x_2 - x_1;
        let numerator = numerator_1 * numerator_2;

        // calculates f(x-2) - f(x-1)
        let denominator_1 = question3(x_2);
        let denominator_2 = question3(x_1);
        let denominator = denominator_1 - denominator_2;

        // this eventually calculates x-1 - ((f(x-1)(x-2 - x-1)) / (f(x-2) - f(x-1))) - secant method
        let result = x_1 - (numerator / denominator);
        x_2 = x_1;
        x_1 = result;
        last_result = result;

        // println!("{}",);
        if print_iter {
            println!(
                "Step/ iteration: {} \t Pi: {} \t\t f(Pi): {}",
                iter,
                last_result,
                question3(last_result)
            );
        }

        iter += 1;
    }

    return last_result;
}

pub fn q3_bisection(interval: (f64, f64), iterations: i8, print_iter: bool) -> f64 {
    let mut a = interval.0;
    let mut b = interval.1;

    let mut last_result: f64 = 0.0;

    let mut iter = 0;
    while iter < iterations {
        let mid = (a + b) / 2.0;
        let result = question3(mid);

        if result < 0.0 {
            b = mid;
            last_result = mid;
        } else if result > 0.0 {
            a = mid;
            last_result = mid;
        } else {
            return mid;
        }

        if print_iter {
            println!(
                "Step/ iteration: {} \t result: {} \t points:{}  {}",
                iter, last_result, a, b
            );
        }

        iter += 1;
    }

    return last_result;
}

pub fn q3_newton(x_1: f64, iterations: i8, print_iter: bool) -> f64 {
    let deriv_func = "2*(x-2) - 1/x";
    let mut x_1 = x_1;
    let mut last_result: f64 = 0.0;

    let mut iter = 0;
    while iter < iterations {
        let numerator = question3(x_1);
        let denominator =
            eval_str(deriv_func.clone().replace("x", x_1.to_string().as_str())).unwrap();

        let result = x_1 - (numerator / denominator);
        x_1 = result;
        last_result = result;

        if print_iter {
            println!(
                "Step/ iteration: {} \t Pi: {} \t\t f(Pi): {}",
                iter,
                last_result,
                question3(last_result)
            );
        }

        iter += 1;
    }

    return last_result;
}
