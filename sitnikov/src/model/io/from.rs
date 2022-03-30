//! Provides the [`from`](Model#method.from) method

use anyhow::{Context, Result};
use numeric_literals::replace_float_literals;

use super::super::{Model, Results};
use crate::cli::Args;
use crate::Float;

#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float> Model<F> {
    /// Initialize a model from arguments
    pub fn from(args: &Args<F>) -> Result<Self> {
        // Define the initial value of time
        //
        // This value is fixed so the place of this zero is known.
        // That's important when computing MEGNOs, because they
        // have a singular point at `t = 0`
        let t_0 = 0.;
        // Prepare a new object
        let mut model = Self {
            e: args.e,
            tau: args.tau * 2. * F::PI(),
            t_0,
            x_0: Vec::new(),
            h: args.h * F::FRAC_PI_2(),
            // Rounded, just in case. The time step validator
            // should prove this to be an integral value
            n: (F::from(args.p).unwrap() * 4. / args.h)
                .round()
                .to_usize()
                .unwrap(),
            // Skip the first quarter of the period
            i_m: (1. / args.h).round().to_usize().unwrap(),
            compute_megnos: args.compute_megnos,
            results: Results::new(),
        };
        // Compute the initial acceleration
        let a_0 = model
            .acceleration(t_0, args.z_0)
            .with_context(|| "Couldn't compute the initial acceleration")?;
        // Set the vector of initial values
        model.x_0 = vec![args.z_0, args.z_v_0, a_0];
        Ok(model)
    }
}
