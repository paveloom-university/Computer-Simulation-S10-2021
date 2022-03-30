//! Provides the [`SymplecticIntegrator`](crate::SymplecticIntegrator) trait

#[doc(hidden)]
mod integrate;
#[doc(hidden)]
mod leapfrog;
#[doc(hidden)]
mod leapfrog_once;
#[doc(hidden)]
mod yoshida_4th;

#[cfg(test)]
mod test_method;
#[cfg(test)]
mod yoshida_4th_2;

use anyhow::{self, Context};
use nalgebra::DVector;
use numeric_literals::replace_float_literals;

use crate::prepare::prepare;
use crate::{Float, Result, ResultExt, Token};

pub(self) use integrate::integrate;
pub(self) use leapfrog::leapfrog;
pub(self) use leapfrog_once::leapfrog_once;
pub(self) use yoshida_4th::yoshida_4th;

#[cfg(test)]
pub(self) use yoshida_4th_2::yoshida_4th_2;

/// Symplectic integrators
pub enum Integrators {
    /// Leapfrog method
    Leapfrog,
    /// 4th-order Yoshida method
    Yoshida4th,
}

/// A symplectic integrator for a system of 1st-order ODEs
pub trait Integrator<F: Float> {
    /// Compute the current values of accelerations as defined
    /// by a system of 2nd-order ODEs, return the result
    ///
    /// Arguments:
    /// * `t` --- Current time moment;
    /// * `x` --- Current values of positions.
    fn accelerations(&self, t: F, x: &[F]) -> anyhow::Result<Vec<F>>;
    // The rest of the methods are defined by these macros
    integrate!();
    leapfrog!();
    leapfrog_once!();
    prepare!();
    yoshida_4th!();
    #[cfg(test)]
    yoshida_4th_2!();
}
