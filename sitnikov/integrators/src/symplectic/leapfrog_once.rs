//! Provides the [`leapfrog_once`] macro

/// Defines the [`leapfrog_once`](crate::SymplecticIntegrator#method.leapfrog_step) method
macro_rules! leapfrog_once {
    () => {
        /// Integrate the system once using the leapfrog method,
        /// return the current state of the system and new accelerations
        ///
        /// Arguments:
        /// * `i` --- Current index of the state;
        /// * `t` --- Current time moment;
        /// * `x` --- Current state of the system;
        /// * `a` --- Current vector of accelerations;
        /// * `h` --- Time step;
        /// * `token` --- Private token.
        #[replace_float_literals(F::from(literal).unwrap())]
        fn leapfrog_once(&self, t: F, x_prev: &[F], h: F, _: &Token) -> anyhow::Result<Vec<F>> {
            // Get the length of the state vector and its thirds
            let l = x_prev.len();
            let lt1 = l / 3;
            let lt2 = 2 * lt1;
            // Create a new vector of state
            let mut x = vec![0.; l];
            // Update the positions
            for j in 0..lt1 {
                x[j] = x_prev[j] + x_prev[j + lt1] * h + 0.5 * x_prev[j + lt2] * h.powi(2)
            }
            // Compute new accelerations
            let a = self
                .accelerations(t + h, &x[0..lt1])
                .with_context(|| "Couldn't compute the new acceleration")?;
            // Update the accelerations and velocities
            for j in lt1..lt2 {
                x[j + lt1] = a[j - lt1];
                x[j] = x_prev[j] + 0.5 * (x_prev[j + lt1] + x[j + lt1]) * h;
            }
            Ok(x)
        }
    };
}

pub(super) use leapfrog_once;
