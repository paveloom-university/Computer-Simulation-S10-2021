//! Provides the [`integrate`] macro

/// Defines the [`integrate`](crate::GeneralIntegrator#method.integrate) method
macro_rules! integrate {
    () => {
        /// Integrate the system of 1st-order ODEs
        ///
        /// Arguments:
        /// * `x` --- Vector of initial values;
        /// * `t_0` --- Initial value of time;
        /// * `h` --- Time step;
        /// * `n` --- Number of iterations;
        /// * `method` --- Integration method.
        #[replace_float_literals(F::from(literal).unwrap())]
        fn integrate(&self, x: Vec<F>, t_0: F, h: F, n: usize, method: Integrators) {
            // Get a token for using the private methods
            let token = Token {};
            // Prepare a result matrix
            let mut result = self.prepare(x, n, &token);
            // Call the specified method to perform integration
            match method {
                Integrators::RungeKutta4th => {
                    self.runge_kutta_4th(t_0, h, n, &mut result, &token);
                }
            }
        }
    };
}

pub(super) use integrate;
