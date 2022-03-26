//! This crate provides an interface for integrating a system of
//! 1st-order ordinary differential equations (ODEs). Specifically,
//! a user can implement either the [`GeneralIntegrator`] trait or
//! the [`SymplecticIntegrator`] trait by defining the system in
//! question and gain a variety of methods for integrating it.

#[doc(hidden)]
mod general;
#[doc(hidden)]
mod symplectic;

#[doc(hidden)]
mod prepare;
#[doc(hidden)]
mod result;

/// Provides a private [`Token`]
mod private {
    /// This struct is used as a type of pseudo-arguments
    /// defined for some of the methods. The reasoning is
    /// simple: forbid reimplementation of those methods
    pub struct Token {}
}

use num::Float as NumFloat;

use std::fmt::{Debug, Display};

use private::Token;

pub use general::{Integrator as GeneralIntegrator, Integrators as GeneralIntegrators};
pub use result::{Ext as ResultExt, Result};
pub use symplectic::{Integrator as SymplecticIntegrator, Integrators as SymplecticIntegrators};

/// A general trait for all floating point type numbers
pub trait Float: 'static + Copy + Debug + Display + NumFloat {}
impl Float for f32 {}
impl Float for f64 {}

/// The biggest floating-point type with implemented [`Float`] trait
type FloatMax = f64;
