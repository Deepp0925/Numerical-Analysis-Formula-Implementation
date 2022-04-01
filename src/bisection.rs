use meval::eval_str;

pub fn bisection(func: &str, interval: (f64, f64), iterations: i8, print_iter: bool) -> f64 {
    let mut a = interval.0;
    let mut b = interval.1;

    let mut last_result: f64 = 0.0;

    let mut iter = 0;
    while iter < iterations {
        let mid = (a + b) / 2.0;
        let new_fn = func.clone().replace("x", mid.to_string().as_str());
        let result = eval_str(new_fn).unwrap();

        if result > 0.0 {
            b = mid;
            last_result = mid;
        } else if result < 0.0 {
            a = mid;
            last_result = mid;
        } else {
            if print_iter {
                println!(
                    "Step/ iteration: {} \t result: {} \t points:{}  {}",
                    iter, mid, a, b
                );
            }
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
