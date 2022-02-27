//! This module provides a method for computing
//! the [acceleration](Model#method.acceleration)
//! (second derivative)

use anyhow::{Context, Result};

use super::super::Model;
use crate::F;

impl Model {
    /// Compute the acceleration (second derivative)
    pub fn acceleration(&self, t: F, z: F) -> Result<F> {
        let r = self
            .radius(t)
            .with_context(|| "Couldn't compute the radius")?;
        Ok(-z / (r.powi(2) + z.powi(2)).powf(1.5))
    }
}
