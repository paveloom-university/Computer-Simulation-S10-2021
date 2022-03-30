//! Provides the [`eccentric_anomaly`](Model#method.eccentric_anomaly) method

use anyhow::{Context, Result};
use numeric_literals::replace_float_literals;

use super::super::Model;
use super::newton_raphson::newton_raphson;
use crate::Float;

impl<F: Float> Model<F> {
    /// Compute the eccentric anomaly from the eccentricity and the mean anomaly
    #[replace_float_literals(F::from(literal).unwrap())]
    pub(super) fn eccentric_anomaly(&self, m: F) -> Result<F> {
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

#[test]
fn test_circular_case() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let model = Model::<f64>::test();

    // Compute the eccentric anomaly
    let e = model.eccentric_anomaly(1.0)?;

    // Compare to the known result
    if (e - 1.).abs() >= f64::EPSILON {
        return Err(anyhow!(
            "The value of the eccentric anomaly is incorrect (radians): 1.0 vs. {e}"
        ));
    };

    Ok(())
}

#[test]
fn test_elliptic_case_small_e() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.00001;

    // Compute the eccentric anomaly
    let m = std::f64::consts::FRAC_PI_6;
    let e_a = model.eccentric_anomaly(m)?;

    // Compare to the known result
    let e_a_0 = 0.523_603_8;
    if (e_a - e_a_0).abs() >= 1e-7 {
        return Err(anyhow!(
            "The value of the eccentric anomaly is incorrect (radians): {e_a_0} vs. {e_a}"
        ));
    };

    Ok(())
}

#[test]
fn test_elliptic_case_moderate_e() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.6;

    // Compute the eccentric anomaly
    let m = std::f64::consts::FRAC_PI_2;
    let e_a = model.eccentric_anomaly(m)?.to_degrees();

    // Compare to the known result
    let e_a_0 = 119.824_323_327_144_34;
    if (e_a - e_a_0).abs() >= f64::EPSILON {
        return Err(anyhow!(
            "The value of the eccentric anomaly is incorrect (degrees): {e_a_0} vs. {e_a}"
        ));
    };

    Ok(())
}

#[test]
fn test_elliptic_case_big_e() -> Result<()> {
    use anyhow::anyhow;

    // Initialize a test model
    let mut model = Model::<f64>::test();
    model.e = 0.9;

    // Compute the eccentric anomaly
    let m = 3. * std::f64::consts::FRAC_PI_2;
    let e_a = model.eccentric_anomaly(m)?.to_degrees();

    // Compare to the known result
    let e_a_0 = 230.315_867_119_592_8;
    if (e_a - e_a_0).abs() >= f64::EPSILON {
        return Err(anyhow!(
            "The value of the eccentric anomaly is incorrect (degrees): {e_a_0} vs. {e_a}"
        ));
    };

    Ok(())
}
