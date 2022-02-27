//! This module provides the [integration](Model#method.integrate) method

use anyhow::{Context, Result};

use super::super::Model;
use crate::F;

impl Model {
    /// Compute the solution by integrating the equation of motion
    pub fn integrate(&mut self) -> Result<()> {
        // Add capacity to the result vectors
        self.results.z = Vec::<F>::with_capacity((self.n + 1) as usize);
        self.results.z_v = Vec::<F>::with_capacity((self.n + 1) as usize);
        // Push the initial values
        self.results.z.push(self.z_0);
        self.results.z_v.push(self.z_v_0);
        // Integrate the system of ordinary differential
        // equations using the 4th-order Yoshida algorithm
        for i in 1..=(self.n as usize) {
            // Compute the time moment
            let t = self.h * F::from(self.n);
            // Compute the next pair
            let (z, z_v) = self
                .yoshida_4th(t, self.results.z[i - 1], self.results.z_v[i - 1])
                .with_context(|| "Couldn't compute the next pair of values")?;
            // Push the results
            self.results.z.push(z);
            self.results.z_v.push(z_v);
        }

        Ok(())
    }
}
