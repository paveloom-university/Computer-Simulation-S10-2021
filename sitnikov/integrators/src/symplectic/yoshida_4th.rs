//! Provides the [`yoshida_4th`] macro, plus tests for the method

use lazy_static::lazy_static;

use crate::FloatMax;

lazy_static! {
    /// The first coefficient in the 4th-order Yoshida method
    pub static ref D_1: FloatMax = 1. / (2. - FloatMax::exp(FloatMax::ln(2.) / 3.));
    /// The second coefficient in the 4th-order Yoshida method
    pub static ref D_2: FloatMax = 1. - 2. * *D_1;
    /// The sum of the first two coefficients in the 4th-order Yoshida method
    pub static ref D_3: FloatMax = *D_1 + *D_2;
}

/// Defines the [`yoshida_4th`](crate::SymplecticIntegrator#method.yoshida_4th) method
macro_rules! yoshida_4th {
    () => {
        /// Integrate the system using the 4th-order Yoshida method
        ///
        /// Arguments:
        /// * `t_0` --- Initial value of time;
        /// * `h` --- Time step;
        /// * `n` --- Number of iterations;
        /// * `result` --- Result matrix;
        /// * `token` --- Private token.
        #[replace_float_literals(F::from(literal).unwrap())]
        fn yoshida_4th(
            &self,
            t_0: F,
            h: F,
            n: usize,
            result: &mut Result<F>,
            token: &Token,
        ) -> anyhow::Result<()> {
            // Compute the increments
            let i_1 = h * F::from(*yoshida_4th::D_1).unwrap();
            let i_2 = h * F::from(*yoshida_4th::D_2).unwrap();
            let i_3 = h * F::from(*yoshida_4th::D_3).unwrap();
            // Get the initial state
            let mut x = result.initial_values();
            // Integrate
            for i in 0..n {
                // Compute the time moment
                let t = t_0 + F::from(i).unwrap() * h;
                // Compute the next states
                for (l, h) in [(0., i_1), (i_1, i_2), (i_3, i_1)] {
                    x = self
                        .leapfrog_once(t + l, &x, h, token)
                        .with_context(|| "Couldn't compute one of the next states")?;
                }
                // Put the new state in the result
                result.set_state(i + 1, x.clone());
            }
            Ok(())
        }
    };
}

pub(super) use yoshida_4th;

#[cfg(test)]
super::test_method::test_method!(yoshida_4th, 4);
