//! This crate provides an interface for integrating a system of
//! 1st-order ordinary differential equations (ODEs). Specifically,
//! a user can implement the [`Integrators`] trait by defining the
//! system in question and gain a variety of methods for integrating it.

#[doc(hidden)]
mod integrate;
#[doc(hidden)]
mod prepare;
#[doc(hidden)]
mod result;
#[doc(hidden)]
mod runge_kutta_4th;

#[cfg(test)]
mod test;

/// Provides a private [`Token`]
mod private {
    /// This struct is used as a type of pseudo-arguments
    /// defined for some of the methods. The reasoning is
    /// simple: forbid reimplementation of those methods
    pub struct Token {}
}

use nalgebra::{DVector, Dynamic, Matrix};
use num::Float as NumFloat;
use numeric_literals::replace_float_literals;

use std::fmt::{Debug, Display};

use private::Token;
pub use result::{Result, ResultExt};

/// A general trait for all floating point type numbers
pub trait Float: 'static + Copy + Debug + Display + NumFloat {}
impl Float for f32 {}
impl Float for f64 {}

/// Methods for numerical integration of 1st-order ODEs
pub enum Methods {
    /// 4th-order Runge-Kutta method
    RungeKutta4th,
}

/// Structures implementing this trait inherit a variety
/// of methods for solving a system of 1st-order ODEs
pub trait Integrators<F: Float> {
    /// Update the current state as defined by a
    /// system of 1st-order ODEs, return the result
    ///
    /// Arguments:
    /// * `t` --- Current time moment;
    /// * `x` --- Current state of the system.
    fn update(&self, t: F, x: &[F]) -> Vec<F>;
    // The rest of the methods are defined by these macros
    integrate!();
    prepare!();
    runge_kutta_4th!();
}
