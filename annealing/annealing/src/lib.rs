//! This crate provides implementations of the
//! [simulated annealing](https://en.wikipedia.org/wiki/Simulated_annealing) and the
//! [adaptive simulated annealing](https://en.wikipedia.org/wiki/Adaptive_simulated_annealing)
//! algorithms for approximating the global minimum of a given function.
//!
//! References:
//! - Jason Brownlee, 2021, “[Simulated Annealing From Scratch in Python](https://machinelearningmastery.com/simulated-annealing-from-scratch-in-python/)”
//! - Mykel J. Kochenderfer, Tim A. Wheeler, 2019, “[Algorithms for Optimization](https://www.amazon.com/dp/0262039427)”
//! - Jonathan Woollett-Light, [`simple_optimization`](https://docs.rs/simple_optimization) crate

#[doc(hidden)]
mod apf;
#[doc(hidden)]
mod neighbour;
#[doc(hidden)]
mod schedule;
#[doc(hidden)]
mod simulated_annealing;

use std::ops::Range;

pub use apf::APF;
pub use neighbour::Method as NeighbourMethod;
pub use schedule::Schedule;
pub use simulated_annealing::SimulatedAnnealing;

/// A point in the parameter (argument) space
pub type Point<F, const N: usize> = [F; N];

/// Bounds of the parameter (argument) space
pub type Bounds<F, const N: usize> = [Range<F>; N];
