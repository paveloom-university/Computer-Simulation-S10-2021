//! This module provides a model of the Sitnikov problem

mod comp;
mod io;

use numeric_literals::replace_float_literals;

use crate::cli::Args;
use crate::Float;

/// A model of the Sitnikov problem
#[derive(Clone)]
pub struct Model<F: Float> {
    /// Eccentricity
    e: F,
    /// Initial value of time
    t_0: F,
    /// Initial value of position of the third body
    z_0: F,
    /// Initial value of velocity of the third body
    z_v_0: F,
    /// Time step
    h: F,
    /// Number of iterations
    n: usize,
    /// Results of the integration
    results: Results<F>,
}

#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float> Model<F> {
    /// Initialize a model from arguments
    pub fn from(args: &Args<F>) -> Self {
        Self {
            e: args.e,
            z_0: args.z_0,
            t_0: args.t_0,
            z_v_0: args.z_v_0,
            h: args.h * F::FRAC_PI_2(),
            // Rounded (just in case) because it's supposed to
            // be integral because of the time step validator
            n: (F::from(args.p).unwrap() * 4. / args.h)
                .round()
                .to_usize()
                .unwrap(),
            results: Results::new(),
        }
    }
}

#[cfg(test)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float> Model<F> {
    /// Initialize a model with default values set for tests
    pub fn test() -> Self {
        let h = 1e-2;
        Self {
            e: 0.,
            t_0: 0.,
            z_0: 1.,
            z_v_0: 0.,
            h: h * F::FRAC_PI_2(),
            n: (1000. * 4. / h).round().to_usize().unwrap(),
            results: Results::new(),
        }
    }
}

/// Results of integration
#[derive(Clone)]
struct Results<F: Float> {
    /// The position of the third body
    z: Vec<F>,
    /// The velocity of the third body
    z_v: Vec<F>,
}

impl<F: Float> Results<F> {
    /// Initialize result vectors
    fn new() -> Self {
        Self {
            z: Vec::<F>::new(),
            z_v: Vec::<F>::new(),
        }
    }
}
