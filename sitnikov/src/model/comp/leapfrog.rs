//! This module provides an implementation of the
//! one-step [leapfrog](Model#method.leapfrog) method

use anyhow::{Context, Result};
use numeric_literals::replace_float_literals;

use super::super::Model;
use crate::Float;

impl<F: Float> Model<F> {
    /// Do a one-step integration using the leapfrog method
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn leapfrog(&self, t: F, z: F, z_v: F, h: F) -> Result<(F, F)> {
        let a_1 = self
            .acceleration(t, z)
            .with_context(|| "Couldn't compute the first acceleration")?;
        let z = z + z_v * h + 0.5 * a_1 * h * h;
        let a_2 = self
            .acceleration(t + h, z)
            .with_context(|| "Couldn't compute the second acceleration")?;
        let z_v = z_v + 0.5 * (a_1 + a_2) * h;
        Ok((z, z_v))
    }
}

#[test]
fn test_time_reversibility() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;
    model.h = 1e-6;

    // Define the number of integrations
    let n = 1000;

    // Put initial values in value holders
    let mut z = model.z_0;
    let mut z_v = model.z_v_0;

    // Integrate forward `n` times and then backward `n` times
    for i in 0..2 * n {
        // Compute the time moment
        let t = if i <= n {
            f64::from(i)
        } else {
            f64::from(2 * n - i)
        } * model.h;
        // Compute the step
        let h = if i < n { model.h } else { -model.h };
        // Compute the next pair of values
        (z, z_v) = model.leapfrog(t, z, z_v, h)?;
    }

    // Compare the results with the initial values
    if (z - model.z_0).abs() >= model.h.powi(2) {
        return Err(anyhow!(
            "The value of the position isn't the same: {} vs. {z}",
            model.z_0
        ));
    }
    if (z_v - model.z_v_0).abs() >= model.h.powi(2) {
        return Err(anyhow!(
            "The value of the velocity isn't the same: {} vs. {z_v}",
            model.z_v_0
        ));
    }

    Ok(())
}
