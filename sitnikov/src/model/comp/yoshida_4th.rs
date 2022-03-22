//! This module provides implementations of the 4th-order
//! [Yoshida](Model#method.yoshida_4th) method

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use numeric_literals::replace_float_literals;

use super::super::Model;
use crate::{Float, FloatMax};

lazy_static! {
    /// The first coefficient in the 4th-order Yoshida method
    static ref D_1: FloatMax = 1. / (2. - FloatMax::exp(FloatMax::ln(2.) / 3.));
    /// The second coefficient in the 4th-order Yoshida method
    static ref D_2: FloatMax = 1. - 2. * *D_1;
     /// The sum of the first two coefficients in the 4th-order Yoshida method
    static ref D_3: FloatMax = *D_1 + *D_2;
}

impl<F: Float> Model<F> {
    /// Integrate the system using the 4th-order Yoshida method
    #[replace_float_literals(F::from(literal).unwrap())]
    pub(super) fn yoshida_4th(&mut self) -> Result<()> {
        // Add capacity to the result vectors
        self.results.z = Vec::<F>::with_capacity(self.n + 1);
        self.results.z_v = Vec::<F>::with_capacity(self.n + 1);
        // Push the initial values
        self.results.z.push(self.z_0);
        self.results.z_v.push(self.z_v_0);
        // Compute the increments
        let i_1 = self.h * F::from(*D_1).unwrap();
        let i_2 = self.h * F::from(*D_2).unwrap();
        let i_3 = self.h * F::from(*D_3).unwrap();
        // Put initial values in value holders
        let mut z = self.z_0;
        let mut z_v = self.z_v_0;
        let mut a = self
            .acceleration(self.t_0, z)
            .with_context(|| "Couldn't compute the initial value of the acceleration")?;
        // Integrate
        for i in 0..self.n {
            // Compute the time moment
            let t = self.t_0 + F::from(i).unwrap() * self.h;
            // Compute the next pair of values
            for (i, h) in [(0., i_1), (i_1, i_2), (i_3, i_1)] {
                (z, z_v, a) = self
                    .leapfrog_step(t + i, z, z_v, h, a)
                    .with_context(|| "The leapfrog method failed")?;
            }
            // Push the results
            self.results.z.push(z);
            self.results.z_v.push(z_v);
        }
        Ok(())
    }
}

#[cfg(test)]
lazy_static! {
    /// The value of $ 2^{\frac{1}{3}} $
    static ref K_2: FloatMax = FloatMax::exp(FloatMax::ln(2.) / 3.);
    /// The first coefficient in the 4th-order Yoshida method (another implementation)
    static ref W_0: FloatMax = -*K_2 / (2. - *K_2);
    /// The second coefficient in the 4th-order Yoshida method (another implementation)
    static ref W_1: FloatMax = 1. / (2. - *K_2);
    /// The half of the first coefficient in the 4th-order Yoshida method (another implementation)
    static ref W_2: FloatMax = *W_1 / 2.;
    /// The half-sum of the first two coefficients in the 4th-order Yoshida method (another implementation)
    static ref W_3: FloatMax = (*W_0 + *W_1) / 2.;
}

#[cfg(test)]
impl<F: Float> Model<F> {
    /// Integrate the system using the 4th-order Yoshida method (another implementation)
    pub fn yoshida_4th_2(&mut self) -> Result<(F, F)> {
        // Add capacity to the result vectors
        self.results.z = Vec::<F>::with_capacity(self.n + 1);
        self.results.z_v = Vec::<F>::with_capacity(self.n + 1);
        // Push the initial values
        self.results.z.push(self.z_0);
        self.results.z_v.push(self.z_v_0);
        // Compute the coefficients
        let c_1 = F::from(*W_2).unwrap() * self.h;
        let c_2 = F::from(*W_3).unwrap() * self.h;
        let c_3 = c_2;
        let c_4 = c_1;
        let d_1 = F::from(*W_1).unwrap() * self.h;
        let d_2 = F::from(*W_0).unwrap() * self.h;
        let d_3 = d_1;
        // Compute the increments
        let i_1 = c_1;
        let i_2 = c_1 + c_2;
        let i_3 = c_1 + c_2 + c_3;
        // Put initial values in value holders
        let mut z = self.z_0;
        let mut z_v = self.z_v_0;
        // Integrate
        for i in 0..self.n {
            // Compute the time moment
            let t = self.t_0 + F::from(i).unwrap() * self.h;
            // Compute the next pair of values
            for (c, d, i) in [(c_1, d_1, i_1), (c_2, d_2, i_2), (c_3, d_3, i_3)] {
                z = z + c * z_v;
                z_v = z_v
                    + d * self
                        .acceleration(t + i, z)
                        .with_context(|| "Couldn't compute the acceleration")?;
            }
            z = z + c_4 * z_v;
            // Push the results
            self.results.z.push(z);
            self.results.z_v.push(z_v);
        }

        Ok((z, z_v))
    }
}

#[test]
fn compare_with_leapfrog() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;
    model.h = 1e-6;
    model.n = 1_000_000;

    // Integrate using the leapfrog method
    model
        .leapfrog()
        .with_context(|| "Couldn't integrate with the leapfrog method")?;

    // Save the results
    let z_1 = *model.results.z.last().unwrap();
    let z_v_1 = *model.results.z_v.last().unwrap();

    // Change the parameters of the model
    model.h = 1e-3;
    model.n = 1000;

    // Integrate using the 4th-order Yoshida method
    model
        .yoshida_4th()
        .with_context(|| "Couldn't integrate with the 4th-order Yoshida method")?;

    // Save the results
    let z_2 = *model.results.z.last().unwrap();
    let z_v_2 = *model.results.z_v.last().unwrap();

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
fn compare_implementations() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;
    model.h = 1e-3;
    model.n = 1000;

    // Integrate using the first implementation
    model.yoshida_4th()?;

    // Save the results
    let z_1 = *model.results.z.last().unwrap();
    let z_v_1 = *model.results.z_v.last().unwrap();

    // Integrate using the second implementation
    model
        .yoshida_4th_2()
        .with_context(|| "Couldn't integrate")?;

    // Save the results
    let z_2 = model.results.z.last().unwrap();
    let z_v_2 = model.results.z_v.last().unwrap();

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
#[allow(clippy::cast_precision_loss)]
fn test_time_reversibility() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;
    model.h = 1e-3;
    model.n = 1000;

    // Save the initial values
    let z_0 = model.z_0;
    let z_v_0 = model.z_v_0;

    // Integrate forward
    model
        .yoshida_4th()
        .with_context(|| "Couldn't integrate forward")?;

    // Change the direction of integration
    model.t_0 = model.n as f64 * model.h;
    model.z_0 = *model.results.z.last().unwrap();
    model.z_v_0 = *model.results.z_v.last().unwrap();
    model.h = -model.h;

    // Integrate backward
    model
        .yoshida_4th()
        .with_context(|| "Couldn't integrate backward")?;

    // Save the results
    let z = *model.results.z.last().unwrap();
    let z_v = *model.results.z_v.last().unwrap();

    // Compare the results with the initial values
    if (z - z_0).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the position isn't the same: {z_0} vs. {z}",
        ));
    }
    if (z_v - z_v_0).abs() >= model.h.powi(4) {
        return Err(anyhow!(
            "The value of the velocity isn't the same: {z_v_0} vs. {z_v}",
        ));
    }

    Ok(())
}
