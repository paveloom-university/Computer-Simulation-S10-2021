//! Provides the [`GeneralIntegrator`](crate::GeneralIntegrator) trait

#[doc(hidden)]
mod integrate;
#[doc(hidden)]
mod runge_kutta_4th;

#[cfg(test)]
mod test_method;

use nalgebra::{DVector, Dynamic, Matrix};
use numeric_literals::replace_float_literals;

use crate::prepare::prepare;
use crate::{Float, Result, ResultExt, Token};

pub(self) use integrate::integrate;
pub(self) use runge_kutta_4th::runge_kutta_4th;

/// General integrators
pub enum Integrators {
    /// 4th-order Runge-Kutta method
    RungeKutta4th,
}

/// A general integrator for a system of 1st-order ODEs
pub trait Integrator<F: Float> {
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
