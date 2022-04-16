#![allow(warnings)]
mod bisection;
mod divdiff;
mod factorial;
// mod gaussian;
mod hermite;
mod lagrange;
mod midpoint;
mod neville;
mod newton;
mod regula;
mod richardson;
mod romberg;
mod secant;
mod simpson;
mod trapazoidal;
use meval::eval_str;
use std::f64::consts::{E, PI};
use std::{io::stdin, str::FromStr};

use crate::bisection::bisection;
use crate::divdiff::{newton_backward_diff, newton_forward_diff};
// use crate::gaussian::gaussian_quadrature;
use crate::hermite::hermite;
use crate::neville::{neville, neville_w_func};
use crate::newton::newton;
use crate::regula::regula_falsi;
use crate::romberg::romberg;
use crate::secant::secant;
use crate::simpson::{composite_simpson, composite_simpson_err, simpson, simpson_err};
use crate::trapazoidal::{
    composite_trapazoidal, composite_trapazoidal_err, trapazoidal, trapazoidal_err,
};
fn main() {
    println!();
    // romberg(E, 2.0 * E, &f, 4);
    // let min_err = 10.0_f64.powi(-4);
    // let n = 451;
    // println!(
    //     "Composite Trapazoidal: {}",
    //     composite_trapazoidal(0.0, PI, &f, n)
    // );
    // println!(
    //     "Composite Trapazoidal Err: {}",
    //     composite_trapazoidal_err(0.0, PI, &f2_deriv, n)
    // );
    // println!(
    //     "is under min Composite Trapazoidal Err: {}",
    //     composite_trapazoidal_err(0.0, PI, &f2_deriv, n) < min_err
    // );
    // println!("n: {} h: {}", n, (PI - 0.0) / n as f64);
    // let n = 51;
    // println!("Composite Simpson: {}", composite_simpson(0.0, PI, &f, n));
    // println!(
    //     "Composite Simpson Err: {}",
    //     composite_simpson_err(0.0, PI, &f4_deriv, n)
    // );
    // println!(
    //     "is under min Comp. Simpson Err: {}",
    //     composite_simpson_err(0.0, PI, &f4_deriv, n) < min_err
    // );
    // println!("n: {} h: {}", n, (PI - 0.0) / n as f64);
    println!();
}

fn f(x: f64) -> f64 {
    return (1.0 / (x * x.ln()));
}

fn f2_deriv(x: f64) -> f64 {
    return (2.0 - x.powi(2)) * x.cos() - 4.0 * x * x.sin();
}

fn f4_deriv(x: f64) -> f64 {
    return 8.0 * x * x.sin() + (x.powi(2) - 12.0) * x.cos();
}
