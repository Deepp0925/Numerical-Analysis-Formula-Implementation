use meval::eval_str;

pub fn secant(func: &str, x_2: f64, x_1: f64, iterations: i8, print_iter: bool) -> f64 {
    let mut x_2 = x_2;
    let mut x_1 = x_1;

    let mut last_result: f64 = 0.0;

    let mut iter = 0;
    while iter < iterations {
        // calculates f(x-1)(x-2 - x-1)
        let numerator_1 = eval_str(func.clone().replace("x", x_1.to_string().as_str())).unwrap();
        let numerator_2 = x_2 - x_1;
        let numerator = numerator_1 * numerator_2;

        // calculates f(x-2) - f(x-1)
        let denominator_1 = eval_str(func.clone().replace("x", x_2.to_string().as_str())).unwrap();
        let denominator_2 = eval_str(func.clone().replace("x", x_1.to_string().as_str())).unwrap();
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
                eval_str(func.clone().replace("x", last_result.to_string().as_str())).unwrap()
            );
        }

        iter += 1;
    }

    return last_result;
}
