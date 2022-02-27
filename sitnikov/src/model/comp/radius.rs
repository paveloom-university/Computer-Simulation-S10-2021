//! This module provides a method for
//! computing the [radius](Model#method.radius)

use anyhow::{Context, Result};

use super::super::Model;
use crate::F;

impl Model {
    /// Compute the radius (distance from the focus to either
    /// of the primary bodies) from the eccentricity and time
    pub fn radius(&self, t: F) -> Result<F> {
        let ea = self
            .eccentric_anomaly(t)
            .with_context(|| "Couldn't compute the eccentric anomaly")?;
        Ok(1. - self.e * F::cos(ea))
    }
}
