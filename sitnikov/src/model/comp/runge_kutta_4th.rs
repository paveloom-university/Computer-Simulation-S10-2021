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
    pub fn runge_kutta_4th(&mut self) -> Result<()> {
        // Add capacity to the result vectors
        self.results.z = Vec::<F>::with_capacity(self.n + 1);
        self.results.z_v = Vec::<F>::with_capacity(self.n + 1);
        // Push the initial values
        self.results.z.push(self.z_0);
        self.results.z_v.push(self.z_v_0);
        // Integrate
        for i in 0..self.n {
            // Compute the time moment
            let t = self.t_0 + F::from(i).unwrap() * self.h;

            // Get the current values
            let z = self.results.z[i];
            let z_v = self.results.z_v[i];

            // Compute the increments
            let k_z_1 = z_v;
            let k_z_v_1 = self.acceleration(t, z).with_context(|| {
                "Couldn't compute the first intermediary value of the acceleration"
            })?;
            let k_z_2 = z_v + self.h * k_z_v_1 / 2.;
            let k_z_v_2 = self
                .acceleration(t + self.h / 2., z + self.h * k_z_1 / 2.)
                .with_context(|| {
                    "Couldn't compute the second intermediary value of the acceleration"
                })?;
            let k_z_3 = z_v + self.h * k_z_v_2 / 2.;
            let k_z_v_3 = self
                .acceleration(t + self.h / 2., z + self.h * k_z_2 / 2.)
                .with_context(|| {
                    "Couldn't compute the third intermediary value of the acceleration"
                })?;
            let k_z_4 = z_v + self.h * k_z_v_3;
            let k_z_v_4 = self
                .acceleration(t + self.h, z + self.h * k_z_3)
                .with_context(|| {
                    "Couldn't compute the fourth intermediary value of the acceleration"
                })?;

            // Compute the results
            let z = z + self.h / 6. * (k_z_1 + 2. * k_z_2 + 2. * k_z_3 + k_z_4);
            let z_v = z_v + self.h / 6. * (k_z_v_1 + 2. * k_z_v_2 + 2. * k_z_v_3 + k_z_v_4);

            // Push the results
            self.results.z.push(z);
            self.results.z_v.push(z_v);
        }
        Ok(())
    }
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn test_time_reversibility() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;
    model.h = 1e-3;
    model.n = 1000;

    // Save the initial values
    let z_0 = model.z_0;
    let z_v_0 = model.z_v_0;

    // Integrate forward
    model
        .runge_kutta_4th()
        .with_context(|| "Couldn't integrate forward")?;

    // Change the direction of integration
    model.t_0 = model.n as f64 * model.h;
    model.z_0 = *model.results.z.last().unwrap();
    model.z_v_0 = *model.results.z_v.last().unwrap();
    model.h = -model.h;

    // Integrate backward
    model
        .runge_kutta_4th()
        .with_context(|| "Couldn't integrate backward")?;

    // Save the results
    let z = *model.results.z.last().unwrap();
    let z_v = *model.results.z_v.last().unwrap();

    // Compare the results with the initial values
    if (z - z_0).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the position isn't the same: {z_0} vs. {z}",
        ));
    }
    if (z_v - z_v_0).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the velocity isn't the same: {z_v_0} vs. {z_v}",
        ));
    }

    Ok(())
}
