use meval::eval_str;

pub fn newton(func: &str, deriv_func: &str, x_1: f64, iterations: i8, print_iter: bool) -> f64 {
    let mut x_1 = x_1;
    let mut last_result: f64 = 0.0;

    let mut iter = 0;
    while iter < iterations {
        let numerator = eval_str(func.clone().replace("x", x_1.to_string().as_str())).unwrap();
        let denominator =
            eval_str(deriv_func.clone().replace("x", x_1.to_string().as_str())).unwrap();

        let result = x_1 - (numerator / denominator);
        x_1 = result;
        last_result = result;

        if print_iter {
            println!(
                "Step/ iteration: {} \t Pi: {} \t\t f(Pi): {} \t f'(Pi): {}",
                iter,
                last_result,
                eval_str(func.clone().replace("x", last_result.to_string().as_str())).unwrap(),
                denominator
            );
        }

        iter += 1;
    }

    return last_result;
}
