//! Provides the [`newton_raphson`] routine

use anyhow::{anyhow, Result};
use numeric_literals::replace_float_literals;

use crate::Float;

/// Maximum number of iterations
const MAX_ITER: u16 = 5000;

/// Find a root of a continuous function using the Newton-Raphson method
#[replace_float_literals(F::from(literal).unwrap())]
pub(super) fn newton_raphson<F: Float>(
    f: impl Fn(F) -> F,
    d: impl Fn(F) -> F,
    initial: F,
) -> Result<F> {
    // If the initial value is already a root
    if initial.abs() < F::epsilon() {
        Ok(initial)
    // Otherwise,
    } else {
        let mut x_1 = initial;
        // On each iteration
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
            // If not, continue
            x_1 = x_2;
        }
        Err(anyhow!(
            "The Newton-Raphson method didn't converge with initial = {initial}"
        ))
    }
}

#[test]
fn test_find_roots() -> Result<()> {
    use anyhow::Context;

    // Define the functions
    let f = |x: f64| x.powi(2) + 3. * x + 2.;
    let d = |x: f64| 2. * x + 3.;

    // Find the roots
    let x_1 = newton_raphson(f, d, -0.85).with_context(|| "Couldn't find the first root")?;
    let x_2 = newton_raphson(f, d, -2.15).with_context(|| "Couldn't find the second root")?;

    // Compare to the known results
    if (x_1 + 1.).abs() >= f64::EPSILON * 10. {
        return Err(anyhow!("The first root is incorrect: -1.0 vs. {x_1}"));
    }
    if (x_2 + 2.).abs() >= f64::EPSILON * 10. {
        return Err(anyhow!("The second root is incorrect: -2.0 vs. {x_2}"));
    }

    Ok(())
}
