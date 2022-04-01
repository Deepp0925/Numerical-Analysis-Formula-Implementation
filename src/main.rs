#![allow(warnings)]
mod bisection;
mod divdiff;
mod factorial;
mod hermite;
mod lagrange;
mod neville;
mod newton;
mod regula;
mod richardson;
mod secant;
use std::{io::stdin, str::FromStr};

// use meval::eval_str;

use meval::eval_str;

use crate::bisection::bisection;
use crate::divdiff::{newton_backward_diff, newton_forward_diff};
use crate::hermite::hermite;
use crate::neville::{neville, neville_w_func};
use crate::newton::newton;
use crate::regula::regula_falsi;
use crate::secant::secant;
fn main() {
    println!();
    richardson::r_extrapolate(1.0, &exp_fn, 0.4, 3);
    println!();
}

fn exp_fn(x: f64) -> f64 {
    x.ln()
}

fn calc_abs_err(actual_ans: f64, approx_ans: f64) -> f64 {
    (actual_ans - approx_ans).abs()
}

fn is_under_err(episolon: f64, val: f64, actual_ans: f64) -> bool {
    let err = (val - actual_ans).abs();
    if err <= episolon {
        return true;
    }
    false
}
