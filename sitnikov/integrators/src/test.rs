//! Provides the [`test`] macro

/// Tests the method for correctness and time reversibility
#[cfg(test)]
#[macro_export]
macro_rules! test {
    ($method:ident) => {
        #[cfg(test)]
        use anyhow::{anyhow, Result};

        #[test]
        #[allow(clippy::cast_precision_loss)]
        fn test() -> Result<()> {
            use crate::private::Token;
            use crate::{Float, Integrators, ResultExt};

            // Implement the trait on a test struct
            type F = f64;
            struct Test {}
            impl<F: Float> Integrators<F> for Test {
                fn update(&self, t: F, x: &[F]) -> Vec<F> {
                    vec![t, x[0] * F::sin(t)]
                }
            }
            let test = Test {};

            // Define the integration parameters
            let x = vec![0., 0.];
            let t_0 = 0.;
            let h = 1e-2;
            let n = 3000;
            let t = t_0 + h * n as f64;
            let token = Token {};

            // Integrate forward, using the 4th-order Runge-Kutta method
            let mut result = test.prepare(x, n, &token);
            test.$method(t_0, h, n, &mut result, &token);

            // Check the results
            let x_0 = vec![
                t.powi(2) / 2.,
                -t.powi(2) / 2. * F::cos(t) + t * F::sin(t) + F::cos(t) - 1.,
            ];
            let x: Vec<F> = result.state(n);
            if x.iter()
                .zip(x_0.iter())
                .any(|(&x, &x_0)| (x - x_0).abs() >= h.powi(4))
            {
                return Err(anyhow!(
                    "The result of integration is not the same as expected: {x_0:?} vs {x:?}"
                ));
            }

            // Integrate forward, using the 4th-order Runge-Kutta method
            test.$method(t, -h, n, &mut result, &token);

            // Check the results
            let x_0 = vec![0., 0.];
            let x: Vec<F> = result.state(0);
            if x.iter()
                .zip(x_0.iter())
                .any(|(&x, &x_0)| (x - x_0).abs() >= h.powi(4))
            {
                return Err(anyhow!(
                    "The integrator doesn't have time reversibility: {x_0:?} vs {x:?}"
                ));
            }

            Ok(())
        }
    };
}
