use meval::eval_str;

pub fn regula_falsi(func: &str, interval: (f64, f64), iterations: i8, print_iter: bool) -> f64 {
    let mut a = interval.0;
    let mut b = interval.1;

    let mut last_result: f64 = 0.0;

    let mut iter = 0;
    while iter < iterations {
        // calculates f(b)
        let numerator_1 = eval_str(func.clone().replace("x", b.to_string().as_str())).unwrap();
        // calculates a * f(b)
        let numerator_1 = numerator_1 * a;

        // calculates f(a)
        let numerator_2 = eval_str(func.clone().replace("x", a.to_string().as_str())).unwrap();
        // calculates f(a) * b
        let numerator_2 = numerator_2 * b;

        // calculates af(b) - bf(a)
        let numerator = numerator_1 - numerator_2;

        // calculates f(b)
        let denominator_1 = eval_str(func.clone().replace("x", b.to_string().as_str())).unwrap();
        // calculates f(a)
        let denominator_2 = eval_str(func.clone().replace("x", a.to_string().as_str())).unwrap();
        // calculates f(b) - f(a)
        let denominator = denominator_1 - denominator_2;

        let result = numerator / denominator;
        // println!("{}", result);
        let eval_result = eval_str(func.clone().replace("x", result.to_string().as_str())).unwrap();
        // println!("{}", result);
        // println!();
        if eval_result < 0.0 {
            b = result;
            last_result = result;
        } else if eval_result > 0.0 {
            a = result;
            last_result = result;
        } else {
            return result;
        }

        if print_iter {
            println!(
                "Step/ iteration: {} \t regula falsi: {} \t f(x) - result: {} \t points:{} {}",
                iter, last_result, eval_result, a, b
            );
        }

        iter += 1;
    }

    return last_result;
}
