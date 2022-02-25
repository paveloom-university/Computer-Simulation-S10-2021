//! This module provides a model of the Sitnikov problem

mod comp;
mod io;

use crate::cli::Args;
use crate::{F, I};

/// A model of the Sitnikov problem
#[derive(Clone)]
pub struct Model {
    /// Eccentricity
    e: F,
    /// Initial value of position of the third body
    z_0: F,
    /// Initial value of velocity of the third body
    z_v_0: F,
    /// Time step
    h: F,
    /// Number of iterations
    n: I,
    /// Results of the integration
    results: Results,
}

impl Model {
    /// Initialize a model
    pub fn from(args: &Args) -> Self {
        Self {
            e: args.e,
            z_0: args.z_0,
            z_v_0: args.z_v_0,
            h: args.h,
            n: args.n,
            results: Results::new(),
        }
    }
}

/// Results of integration
#[derive(Clone)]
struct Results {
    /// The position of the third body
    z: Vec<F>,
    /// The velocity of the third body
    z_v: Vec<F>,
}

impl Results {
    /// Initialize result vectors
    fn new() -> Self {
        Self {
            z: Vec::<F>::new(),
            z_v: Vec::<F>::new(),
        }
    }
}
