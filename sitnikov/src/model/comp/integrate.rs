//! This module provides the integration routine

use anyhow::{Context, Result};

use super::super::Model;
use crate::F;

impl Model {
    // Define the right-hand side of the original
    // second-order differential equation
    fn rhs(&self, t: F, z: F) -> Result<F> {
        let r = self
            .radius(t)
            .with_context(|| "Couldn't compute the radius")?;
        Ok(-z / (r.powi(2) + z.powi(2)).powf(1.5))
    }
    /// Compute the solution by integrating the equation of motion
    pub fn integrate(&mut self) -> Result<()> {
        // Add capacity to the result vectors
        self.results.z = Vec::<F>::with_capacity(self.n as usize + 1);
        self.results.z_v = Vec::<F>::with_capacity(self.n as usize + 1);
        // Push the initial values
        self.results.z.push(self.z_0);
        self.results.z_v.push(self.z_v_0);
        // Integrate the system of ordinary differential
        // equations using the 4th-order Runge-Kutta method
        for i in 1..=(self.n as usize) {
            // Compute the time moment
            let t = self.h * F::from(self.n);

            // Compute a set of increments for each variable

            let k_z_1 = self.results.z_v[i - 1];
            let k_z_v_1 = self.rhs(t, self.results.z[i - 1]).with_context(|| {
                "Couldn't compute the right-hand side of the differential equation"
            })?;

            let k_z_2 = self.results.z_v[i - 1] + self.h * k_z_1 / 2.;
            let k_z_v_2 = self
                .rhs(
                    t + self.h / 2.,
                    self.results.z[i - 1] + self.h * k_z_v_1 / 2.,
                )
                .with_context(|| {
                    "Couldn't compute the right-hand side of the differential equation"
                })?;

            let k_z_3 = self.results.z_v[i - 1] + self.h * k_z_2 / 2.;
            let k_z_v_3 = self
                .rhs(
                    t + self.h / 2.,
                    self.results.z[i - 1] + self.h * k_z_v_2 / 2.,
                )
                .with_context(|| {
                    "Couldn't compute the right-hand side of the differential equation"
                })?;

            let k_z_4 = self.results.z_v[i - 1] + self.h * k_z_3;
            let k_z_v_4 = self
                .rhs(t + self.h, self.results.z[i - 1] + self.h * k_z_v_3)
                .with_context(|| {
                    "Couldn't compute the right-hand side of the differential equation"
                })?;

            // Compute and push the results of this iteration

            self.results
                .z
                .push(self.results.z[i - 1] + self.h / 6. * (k_z_1 + k_z_2 + k_z_3 + k_z_4));

            self.results.z_v.push(
                self.results.z_v[i - 1] + self.h / 6. * (k_z_v_1 + k_z_v_2 + k_z_v_3 + k_z_v_4),
            );
        }

        Ok(())
    }
}
