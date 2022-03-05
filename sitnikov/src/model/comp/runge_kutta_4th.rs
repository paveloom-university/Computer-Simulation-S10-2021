//! This module provides an implementation of the one-step
//! 4th-order [Runge-Kutta](Model#method.runge_kutta) method

use anyhow::{Context, Result};
use numeric_literals::replace_float_literals;

use super::super::Model;
use crate::Float;

impl<F: Float> Model<F> {
    /// Do a one-step integration using the 4th-order Runge-Kutta method
    #[allow(dead_code)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn runge_kutta_4th(&self, t: F, z: F, z_v: F, h: F) -> Result<(F, F)> {
        let k_z_1 = z_v;
        let k_z_v_1 = self
            .acceleration(t, z)
            .with_context(|| "Couldn't compute the first intermediary value of the acceleration")?;

        let k_z_2 = z_v + h * k_z_v_1 / 2.;
        let k_z_v_2 = self
            .acceleration(t + h / 2., z + h * k_z_1 / 2.)
            .with_context(|| {
                "Couldn't compute the second intermediary value of the acceleration"
            })?;

        let k_z_3 = z_v + h * k_z_v_2 / 2.;
        let k_z_v_3 = self
            .acceleration(t + h / 2., z + h * k_z_2 / 2.)
            .with_context(|| "Couldn't compute the third intermediary value of the acceleration")?;

        let k_z_4 = z_v + h * k_z_v_3;
        let k_z_v_4 = self.acceleration(t + h, z + h * k_z_3).with_context(|| {
            "Couldn't compute the fourth intermediary value of the acceleration"
        })?;

        let z = z + h / 6. * (k_z_1 + 2. * k_z_2 + 2. * k_z_3 + k_z_4);
        let z_v = z_v + h / 6. * (k_z_v_1 + 2. * k_z_v_2 + 2. * k_z_v_3 + k_z_v_4);

        Ok((z, z_v))
    }
}

#[test]
fn test_time_reversibility() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;
    model.h = 1e-3;

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
        (z, z_v) = model.runge_kutta_4th(t, z, z_v, h)?;
    }

    // Compare the results with the initial values
    if (z - model.z_0).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the position isn't the same: {} vs. {z}",
            model.z_0
        ));
    }
    if (z_v - model.z_v_0).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the velocity isn't the same: {} vs. {z_v}",
            model.z_v_0
        ));
    }

    Ok(())
}
