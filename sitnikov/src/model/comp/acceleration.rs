//! This module provides a method for computing
//! the [acceleration](Model#method.acceleration)
//! (second derivative)

use anyhow::{Context, Result};
use numeric_literals::replace_float_literals;

use super::super::Model;
use crate::Float;

impl<F: Float> Model<F> {
    /// Compute the acceleration (second derivative)
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn acceleration(&self, t: F, z: F) -> Result<F> {
        let r = self
            .radius(t)
            .with_context(|| "Couldn't compute the radius")?;
        Ok(-z / (r.powi(2) + z.powi(2)).powf(1.5))
    }
}

#[test]
fn test_acceleration() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;

    // Compute the acceleration
    let a = model.acceleration(std::f64::consts::FRAC_PI_2, 1.0)?;

    // Compare to the known result
    let a_0 = -0.227_182_975_639_198_54;
    if (a - a_0).abs() >= f64::EPSILON {
        return Err(anyhow!(
            "The value of the acceleration is incorrect: {a_0} vs {a}"
        ));
    }
    Ok(())
}
