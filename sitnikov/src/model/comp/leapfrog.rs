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
