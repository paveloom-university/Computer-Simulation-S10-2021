//! This module provides an implementation
//! of the one-step 4th-order
//! [Yoshida](Model#method.yoshida_4th) method

use anyhow::{Context, Result};
use numeric_literals::replace_float_literals;

use super::super::Model;
use crate::Float;

#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float> Model<F> {
    /// Do a one-step integration using the 4th-order Yoshida method
    pub fn yoshida_4th(&self, t: F, z: F, z_v: F) -> Result<(F, F)> {
        let d_1 = 1. / (2. - F::exp(F::ln(2.) / 3.));
        let d_2 = 1. - 2. * d_1;
        let (z, z_v) = self
            .leapfrog(t, z, z_v, self.h * d_1)
            .with_context(|| "Failed at the first call of the leapfrog method")?;
        let (z, z_v) = self
            .leapfrog(t, z, z_v, self.h * d_2)
            .with_context(|| "Failed at the second call of the leapfrog method")?;
        let (z, z_v) = self
            .leapfrog(t, z, z_v, self.h * d_1)
            .with_context(|| "Failed at the third call of the leapfrog method")?;
        Ok((z, z_v))
    }
}
