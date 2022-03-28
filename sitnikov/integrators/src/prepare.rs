//! Provides the [`prepare`] macro, plus tests for the method

/// Defines the `prepare` method
macro_rules! prepare {
    () => {
        /// Prepare a matrix for the result and
        /// put the initial values in the first column
        ///
        /// Arguments:
        /// * `x` --- Vector of initial values;
        /// * `n` --- Number of iterations;
        /// * `token` --- Private token.
        #[replace_float_literals(F::from(literal).unwrap())]
        fn prepare(&self, x: Vec<F>, n: usize, _: &Token) -> Result<F> {
            // Define the number of rows
            let nrows = Dynamic::new(x.len());
            // Define the number of columns
            let ncols = Dynamic::new(n + 1);
            // Create a matrix for the solution
            let mut result = Matrix::zeros_generic(nrows, ncols);
            // Wrap the initial values in a column vector
            let x = DVector::from(x);
            // Put the initial values in the first row
            result.set_column(0, &x);
            // Return the matrix
            result
        }
    };
}

pub(super) use prepare;

#[test]
fn test() -> anyhow::Result<()> {
    use crate::private::Token;
    use crate::{Float, GeneralIntegrator, ResultExt};

    // Implement the trait on a test struct
    type F = f64;
    struct Test {}
    impl<F: Float> GeneralIntegrator<F> for Test {
        fn update(&self, _t: F, x: &[F]) -> anyhow::Result<Vec<F>> {
            Ok(x.to_vec())
        }
    }
    let test = Test {};
    let token = Token {};

    // Prepare a matrix with initial data
    let x = vec![1., 2., 3., 4., 5.];
    let x_0 = x.clone();
    let result = test.prepare(x, 0, &token);

    // Check the first column of the matrix
    let x: Vec<F> = result.initial_values();
    if x != x_0 {
        return Err(anyhow::anyhow!(
            "The first column of the matrix is not the same as initial values"
        ));
    }

    Ok(())
}
