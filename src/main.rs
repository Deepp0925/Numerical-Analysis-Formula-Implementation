#![allow(warnings)]
mod bisection;
mod divdiff;
mod factorial;
mod gaussian;
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
use crate::gaussian::gaussian_quadrature;
use crate::hermite::hermite;
use crate::neville::{neville, neville_w_func};
use crate::newton::newton;
use crate::regula::regula_falsi;
use crate::romberg::romberg;
use crate::secant::secant;
use crate::simpson::composite_simpson;
use crate::trapazoidal::composite_trapazoidal;
fn main() {
    println!();
    // romberg(0.0, PI, &f, 4);
    println!("{}", gaussian_quadrature(-1.0, 1.0, &f, 3));
    println!();
}

fn f(x: f64) -> f64 {
    return x.exp() * x.cos();
}
