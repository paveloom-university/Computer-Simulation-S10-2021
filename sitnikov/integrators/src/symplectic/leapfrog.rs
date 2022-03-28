//! Provides the [`leapfrog`] macro, plus tests for the method

/// Defines the [`leapfrog`](crate::SymplecticIntegrator#method.leapfrog) method
macro_rules! leapfrog {
    () => {
        /// Integrate the system using the leapfrog method
        ///
        /// Arguments:
        /// * `t_0` --- Initial value of time;
        /// * `h` --- Time step;
        /// * `n` --- Number of iterations;
        /// * `result` --- Result matrix;
        /// * `token` --- Private token.
        #[replace_float_literals(F::from(literal).unwrap())]
        fn leapfrog(
            &self,
            t_0: F,
            h: F,
            n: usize,
            result: &mut Result<F>,
            token: &Token,
        ) -> anyhow::Result<()> {
            // Get the initial state
            let mut x = result.initial_values();
            // Compute the initial accelerations
            let mut a = self
                .accelerations(t_0, &x)
                .with_context(|| "Couldn't compute the accelerations")?;
            // Integrate
            for i in 0..n {
                // Compute the time moment
                let t = t_0 + F::from(i).unwrap() * h;
                // Compute the next state
                (x, a) = self
                    .leapfrog_once(t, &x, &a, h, token)
                    .with_context(|| "Couldn't compute the next state")?;
                // Put the new state in the result
                result.set_state(i + 1, x.clone());
            }
            Ok(())
        }
    };
}

pub(super) use leapfrog;

#[cfg(test)]
super::test_method::test_method!(leapfrog, 2);
