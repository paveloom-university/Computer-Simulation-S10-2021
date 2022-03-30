//! This module provides a model of the Sitnikov problem

mod comp;
mod io;

use integrators::ResultExt;

#[cfg(test)]
use numeric_literals::replace_float_literals;

use crate::Float;

/// A model of the Sitnikov problem
#[derive(Clone)]
pub struct Model<F: Float> {
    /// Eccentricity
    e: F,
    /// Time at the pericenter
    tau: F,
    /// Initial value of time
    t_0: F,
    /// Vector of initial values
    x_0: Vec<F>,
    /// Time step
    h: F,
    /// Number of iterations
    n: usize,
    /// An index of the first value for MEGNOs
    i_m: usize,
    /// Compute MEGNOs?
    compute_megnos: bool,
    /// Results of the integration
    results: Results<F>,
}

#[cfg(test)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float> Model<F> {
    /// Initialize a model with default values set for tests
    pub fn test() -> Self {
        let h = 1e-2;
        Self {
            e: 0.,
            tau: 0.,
            t_0: 0.,
            x_0: Vec::new(),
            h: h * F::FRAC_PI_2(),
            n: (1000. * 4. / h).round().to_usize().unwrap(),
            i_m: 0,
            compute_megnos: false,
            results: Results::new(),
        }
    }
}

/// Results of integration
#[derive(Clone)]
struct Results<F: Float> {
    /// The integrated trajectory (-ies)
    x: integrators::Result<F>,
    /// The integrated trajectories, MEGNOs and mean MEGNOs
    m: integrators::Result<F>,
}

impl<F: Float> Results<F> {
    /// Initialize result matrices
    fn new() -> Self {
        Self {
            x: integrators::Result::<F>::new(0, 0),
            m: integrators::Result::<F>::new(0, 0),
        }
    }
}
