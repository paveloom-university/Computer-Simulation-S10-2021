//! This module provides a model of the Sitnikov problem

// Both of these come from the line that defines `n`
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

mod comp;
mod io;

use crate::cli::Args;
use crate::{consts, F, I};

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
            // Rounded because it's supposed to be
            // integral because of the time step validator
            n: (F::from(args.t) * 2. * consts::PI / args.h).round() as I,
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
