//! Provides the [`leapfrog_once`] macro

/// Defines the [`leapfrog_once`](crate::SymplecticIntegrator#method.leapfrog_step) method
macro_rules! leapfrog_once {
    () => {
        /// Integrate the system once using the leapfrog method,
        /// return the current state of the system and new accelerations
        ///
        /// Arguments:
        /// * `t` --- Current time moment;
        /// * `x` --- Current state of the system;
        /// * `a` --- Current vector of accelerations;
        /// * `h` --- Time step;
        /// * `token` --- Private token.
        #[replace_float_literals(F::from(literal).unwrap())]
        fn leapfrog_once(
            &self,
            t: F,
            x_prev: &[F],
            a_prev: &[F],
            h: F,
            _: &Token,
        ) -> anyhow::Result<(Vec<F>, Vec<F>)> {
            // Get the length of the state vector and its half
            let l = x_prev.len();
            let lh = l / 2;
            // Create a new vector of state
            let mut x = vec![0.; l];
            // Update the positions
            for i in 0..lh {
                x[i] = x_prev[i] + x_prev[i + lh] * h + 0.5 * a_prev[i] * h.powi(2)
            }
            // Compute new accelerations
            let a = self
                .accelerations(t + h, &x[0..lh])
                .with_context(|| "Couldn't compute the new acceleration")?;
            // Update the velocities
            for i in lh..l {
                x[i] = x_prev[i] + 0.5 * (a_prev[i - lh] + a[i - lh]) * h
            }
            Ok((x, a))
        }
    };
}

pub(super) use leapfrog_once;
