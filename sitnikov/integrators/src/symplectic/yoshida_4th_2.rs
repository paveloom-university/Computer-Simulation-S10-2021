use lazy_static::lazy_static;

use crate::FloatMax;

lazy_static! {
    /// The value of $ 2^{\frac{1}{3}} $
    pub static ref K_2: FloatMax = FloatMax::exp(FloatMax::ln(2.) / 3.);
    /// The first coefficient in the 4th-order Yoshida method (another implementation)
    pub static ref W_0: FloatMax = -*K_2 / (2. - *K_2);
    /// The second coefficient in the 4th-order Yoshida method (another implementation)
    pub static ref W_1: FloatMax = 1. / (2. - *K_2);
    /// The half of the first coefficient in the 4th-order Yoshida method (another implementation)
    pub static ref W_2: FloatMax = *W_1 / 2.;
    /// The half-sum of the first two coefficients in the 4th-order Yoshida method (another implementation)
    pub static ref W_3: FloatMax = (*W_0 + *W_1) / 2.;
}

/// Defines the [`yoshida_4th_2`](crate::SymplecticIntegrator#method.yoshida_4th_2) method
#[cfg(test)]
macro_rules! yoshida_4th_2 {
    () => {
        /// Integrate the system using the 4th-order Yoshida method
        /// (another implementation)
        ///
        /// Arguments:
        /// * `t_0` --- Initial value of time;
        /// * `h` --- Time step;
        /// * `n` --- Number of iterations;
        /// * `result` --- Result matrix;
        /// * `token` --- Private token.
        #[replace_float_literals(F::from(literal).unwrap())]
        fn yoshida_4th_2(
            &self,
            t_0: F,
            h: F,
            n: usize,
            result: &mut Result<F>,
            _: &Token,
        ) -> anyhow::Result<()> {
            // Compute the coefficients
            let c_1 = F::from(*yoshida_4th_2::W_2).unwrap() * h;
            let c_2 = F::from(*yoshida_4th_2::W_3).unwrap() * h;
            let c_3 = c_2;
            let c_4 = c_1;
            let d_1 = F::from(*yoshida_4th_2::W_1).unwrap() * h;
            let d_2 = F::from(*yoshida_4th_2::W_0).unwrap() * h;
            let d_3 = d_1;
            // Compute the increments
            let i_1 = c_1;
            let i_2 = c_1 + c_2;
            let i_3 = c_1 + c_2 + c_3;
            // Get the initial state
            let mut x = result.initial_values();
            // Get the length of the state vector and its half
            let l = x.len();
            let lh = l / 2;
            // Integrate
            for i in 0..n {
                // Compute the time moment
                let t = t_0 + F::from(i).unwrap() * h;
                // Compute the next states
                for (c, d, i) in [(c_1, d_1, i_1), (c_2, d_2, i_2), (c_3, d_3, i_3)] {
                    // Update the positions
                    for i in 0..lh {
                        x[i] = x[i] + c * x[i + lh];
                    }
                    // Compute the accelerations
                    let a = self
                        .accelerations(t + i, &x[0..lh])
                        .with_context(|| "Couldn't compute the accelerations")?;
                    // Update the velocities
                    for i in lh..l {
                        x[i] = x[i] + d * a[i - lh];
                    }
                }
                // Update the positions for the last time
                for i in 0..lh {
                    x[i] = x[i] + c_4 * x[i + lh];
                }
                // Put the new state in the result
                result.set_state(i + 1, x.clone());
            }
            Ok(())
        }
    };
}

pub(super) use yoshida_4th_2;

#[cfg(test)]
super::test_method::test_method!(yoshida_4th_2, 4);
