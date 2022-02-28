//! This module provides a method for
//! computing the [radius](Model#method.radius)

use anyhow::{Context, Result};

use super::super::Model;
use crate::{consts, F};

impl Model {
    /// Compute the radius (distance from the focus to either
    /// of the primary bodies) from the eccentricity and time
    pub fn radius(&self, t: F) -> Result<F> {
        let e_a = self
            .eccentric_anomaly(t % (2. * consts::PI))
            .with_context(|| "Couldn't compute the eccentric anomaly")?;
        Ok((1. - self.e * F::cos(e_a)) / 2.)
    }
}
