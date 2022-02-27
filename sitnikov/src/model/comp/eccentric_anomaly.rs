//! This module provides a method for computing the
//! [eccentric anomaly](Model#method.eccentric_anomaly)

use anyhow::{Context, Result};

use super::super::Model;
use super::newton_raphson::newton_raphson;
use crate::{consts, F};

impl Model {
    /// Compute the eccentric anomaly from the eccentricity and the mean anomaly
    pub fn eccentric_anomaly(&self, m: F) -> Result<F> {
        // Define the non-linear equation
        let fun = |x| x - self.e * F::sin(x) - m;
        // Define its derivative
        let der = |x| 1. - self.e * F::cos(x);
        // Compute the solution
        if self.e == 0. {
            Ok(m)
        } else {
            // Define the initial value
            let initial = if self.e > 0.8 { consts::PI } else { m };
            // Use the Newton–Raphson method as a root-finding algorithm
            newton_raphson(fun, der, initial).with_context(|| "Couldn't find the root")
        }
    }
}
