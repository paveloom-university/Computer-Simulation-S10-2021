//! This module provides the [integration](Model#method.integrate) method

use anyhow::{Context, Result};

use super::super::Model;
use crate::Float;

impl<F: Float> Model<F> {
    /// Compute the solution by integrating the equation of motion
    pub fn integrate(&mut self) -> Result<()> {
        // Add capacity to the result vectors
        self.results.z = Vec::<F>::with_capacity(self.n + 1);
        self.results.z_v = Vec::<F>::with_capacity(self.n + 1);
        // Push the initial values
        self.results.z.push(self.z_0);
        self.results.z_v.push(self.z_v_0);
        // Integrate the system of ordinary differential
        // equations using the 4th-order Yoshida algorithm
        for i in 0..self.n {
            // Compute the time moment
            let t = F::from(i).unwrap() * self.h;
            // Compute the next pair
            let (z, z_v) = self
                .yoshida_4th(t, self.results.z[i], self.results.z_v[i], self.h)
                .with_context(|| "Couldn't compute the next pair of values")?;
            // Push the results
            self.results.z.push(z);
            self.results.z_v.push(z_v);
        }

        Ok(())
    }
}
