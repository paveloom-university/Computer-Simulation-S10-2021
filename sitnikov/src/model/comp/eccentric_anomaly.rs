//! This module provides a method for computing the
//! [eccentric anomaly](Model#method.eccentric_anomaly)

use anyhow::{Context, Result};
use numeric_literals::replace_float_literals;

use super::super::Model;
use super::newton_raphson::newton_raphson;
use crate::Float;

impl<F: Float> Model<F> {
    /// Compute the eccentric anomaly from the eccentricity and the mean anomaly
    #[replace_float_literals(F::from(literal).unwrap())]
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
            let initial = if self.e > 0.8 { F::PI() } else { m };
            // Use the Newton–Raphson method as a root-finding algorithm
            newton_raphson(fun, der, initial).with_context(|| "Couldn't find the root")
        }
    }
}
