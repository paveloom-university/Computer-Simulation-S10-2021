//! This module provides an implementation of the
//! [leapfrog](Model#method.leapfrog) method

use anyhow::{Context, Result};
use numeric_literals::replace_float_literals;

use super::super::Model;
use crate::Float;

impl<F: Float> Model<F> {
    /// Do a one-step integration using the leapfrog method
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn leapfrog_step(&self, t: F, z: F, z_v: F, h: F, a_prev: F) -> Result<(F, F, F)> {
        let z = z + z_v * h + 0.5 * a_prev * h.powi(2);
        let a = self
            .acceleration(t + h, z)
            .with_context(|| "Couldn't compute the next value of the acceleration")?;
        let z_v = z_v + 0.5 * (a_prev + a) * h;
        Ok((z, z_v, a))
    }

    /// Integrate the system using the leapfrog method
    #[cfg(test)]
    pub fn leapfrog(&mut self) -> Result<()> {
        // Add capacity to the result vectors
        self.results.z = Vec::<F>::with_capacity(self.n + 1);
        self.results.z_v = Vec::<F>::with_capacity(self.n + 1);
        // Push the initial values
        self.results.z.push(self.z_0);
        self.results.z_v.push(self.z_v_0);
        // Put initial values in value holders
        let mut z = self.z_0;
        let mut z_v = self.z_v_0;
        let mut a = self
            .acceleration(self.t_0, z)
            .with_context(|| "Couldn't compute the initial acceleration")?;
        // Integrate
        for i in 0..self.n {
            // Compute the time moment
            let t = self.t_0 + F::from(i).unwrap() * self.h;
            // Compute the next pair of values
            (z, z_v, a) = self.leapfrog_step(t, z, z_v, self.h, a)?;
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
    model.h = 1e-6;
    model.n = 1000;

    // Save the initial values
    let z_0 = model.z_0;
    let z_v_0 = model.z_v_0;

    // Integrate forward
    model
        .leapfrog()
        .with_context(|| "Couldn't integrate forward")?;

    // Change the direction of integration
    model.t_0 = model.n as f64 * model.h;
    model.z_0 = *model.results.z.last().unwrap();
    model.z_v_0 = *model.results.z_v.last().unwrap();
    model.h = -model.h;

    // Integrate backward
    model
        .leapfrog()
        .with_context(|| "Couldn't integrate backward")?;

    // Save the results
    let z = *model.results.z.last().unwrap();
    let z_v = *model.results.z_v.last().unwrap();

    // Compare the results with the initial values
    if (z - z_0).abs() >= model.h.powi(2) {
        return Err(anyhow!(
            "The value of the position isn't the same: {z_0} vs. {z}",
        ));
    }
    if (z_v - z_v_0).abs() >= model.h.powi(2) {
        return Err(anyhow!(
            "The value of the velocity isn't the same: {z_v_0} vs. {z_v}",
        ));
    }

    Ok(())
}
