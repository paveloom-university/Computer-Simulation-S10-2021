//! This module provides a general integration method

use anyhow::{Context, Result};

use super::super::Model;
use crate::Float;

impl<F: Float> Model<F> {
    // Integrate the equations of motion and
    // (optionally) compute MEGNOs
    pub(crate) fn integrate(&mut self) -> Result<()> {
        // Integrate the equations of motion
        // using the 4th-order Yoshida method
        self.yoshida_4th()
            .with_context(|| "Couldn't integrate the equations of motion")?;
        // Compute MEGNOs
        if self.compute_megnos {
            self.compute_megnos()
                .with_context(|| "Couldn't compute MEGNOs")?;
        }
        Ok(())
    }
}
