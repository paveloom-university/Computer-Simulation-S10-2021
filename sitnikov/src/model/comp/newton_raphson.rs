//! This module provides an implementation of the Newton-Raphson method

use anyhow::{anyhow, Result};
use numeric_literals::replace_float_literals;

use crate::Float;

/// Maximum number of iterations
const MAX_ITER: u16 = 5000;

/// Find a root of a continuous function using the Newton-Raphson method
#[replace_float_literals(F::from(literal).unwrap())]
pub fn newton_raphson<F: Float>(f: impl Fn(F) -> F, d: impl Fn(F) -> F, initial: F) -> Result<F> {
    // If the initial value is already a root
    if initial < F::epsilon() {
        Ok(initial)
    // Otherwise,
    } else {
        let mut x_1 = initial;
        for _ in 0..MAX_ITER {
            // Compute the function and derivative values
            let f = f(x_1);
            let d = d(x_1);
            // Compute the next point
            let x_2 = x_1 - f / d;
            // Check if the last two points are close enough
            if (x_1 - x_2).abs() < F::epsilon() * 10. {
                return Ok(x_2);
            }
            // If not, repeat
            x_1 = x_2;
        }
        Err(anyhow!(
            "The Newton-Raphson method didn't converge with initial = {initial}"
        ))
    }
}
