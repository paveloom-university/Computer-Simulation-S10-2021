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
    pub fn yoshida_4th(&self, t: F, z: F, z_v: F, h: F) -> Result<(F, F)> {
        let d_1 = 1. / (2. - F::exp(F::ln(2.) / 3.));
        let d_2 = 1. - 2. * d_1;
        let (z, z_v) = self
            .leapfrog(t, z, z_v, h * d_1)
            .with_context(|| "Failed at the first call of the leapfrog method")?;
        let (z, z_v) = self
            .leapfrog(t + h * d_1, z, z_v, h * d_2)
            .with_context(|| "Failed at the second call of the leapfrog method")?;
        let (z, z_v) = self
            .leapfrog(t + h * (d_1 + d_2), z, z_v, h * d_1)
            .with_context(|| "Failed at the third call of the leapfrog method")?;

        Ok((z, z_v))
    }
}

#[cfg(test)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float> Model<F> {
    /// Do a one-step integration using the 4th-order Yoshida method
    pub fn yoshida_4th_explicit(&self, t: F, z: F, z_v: F, h: F) -> Result<(F, F)> {
        let k = F::exp(F::ln(2.) / 3.);
        let w_0 = -k / (2. - k);
        let w_1 = 1. / (2. - k);
        let c_1 = w_1 / 2.;
        let c_2 = (w_0 + w_1) / 2.;
        let c_3 = c_2;
        let c_4 = c_1;
        let d_1 = w_1;
        let d_2 = w_0;
        let d_3 = w_1;

        let mut z = z;
        let mut z_v = z_v;

        for (c, d, e) in [
            (c_1, d_1, c_1),
            (c_2, d_2, c_1 + c_2),
            (c_3, d_3, c_1 + c_2 + c_3),
        ] {
            z = z + c * z_v * h;
            z_v = z_v + d * self.acceleration(t + e * h, z)? * h;
        }

        z = z + c_4 * z_v * h;

        Ok((z, z_v))
    }
}

#[test]
fn test_compare_implementations() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;
    model.h = 1e-3;

    // Define the number of integrations
    let n = 1000;

    // Put initial values in value holders
    let mut z_1 = model.z_0;
    let mut z_v_1 = model.z_v_0;
    let mut z_2 = z_1;
    let mut z_v_2 = z_v_1;

    // Integrate using both implementations
    for i in 0..2 * n {
        // Compute the time moment
        let t = f64::from(i) * model.h;
        // Compute the next pair of values
        (z_1, z_v_1) = model.yoshida_4th(t, z_1, z_v_1, model.h)?;
        (z_2, z_v_2) = model.yoshida_4th_explicit(t, z_2, z_v_2, model.h)?;
    }

    // Compare the results between implementations
    if (z_1 - z_2).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the position isn't the same: {z_1} vs. {z_2}",
        ));
    }
    if (z_v_1 - z_v_2).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the velocity isn't the same: {z_v_1} vs. {z_v_2}",
        ));
    }

    Ok(())
}

#[test]
fn test_time_reversibility() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;
    model.h = 1e-3;

    // Define the number of integrations
    let n = 1000;

    // Put initial values in value holders
    let mut z = model.z_0;
    let mut z_v = model.z_v_0;

    // Integrate forward `n` times and then backward `n` times
    for i in 0..2 * n {
        // Compute the time moment
        let t = if i <= n {
            f64::from(i)
        } else {
            f64::from(2 * n - i)
        } * model.h;
        // Compute the step
        let h = if i < n { model.h } else { -model.h };
        // Compute the next pair of values
        (z, z_v) = model.yoshida_4th(t, z, z_v, h)?;
    }

    // Compare the results with the initial values
    if (z - model.z_0).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the position isn't the same: {} vs. {z}",
            model.z_0
        ));
    }
    if (z_v - model.z_v_0).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the velocity isn't the same: {} vs. {z_v}",
            model.z_v_0
        ));
    }

    Ok(())
}
