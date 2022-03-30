//! This binary crate is a command-line utility for simulating the
//! [Sitnikov problem](https://en.wikipedia.org/wiki/Sitnikov_problem).

mod cli;
mod model;

use anyhow::{Context, Result};
use integrators::Float as IntegratorsFloat;
use num::{traits::FloatConst, Float as NumFloat, NumCast};
use serde::Serialize;

use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::num::ParseFloatError;
use std::str::FromStr;

/// A general trait for all floating point type numbers
pub trait Float:
    Copy
    + Debug
    + Display
    + FloatConst
    + FromStr<Err = ParseFloatError>
    + NumCast
    + NumFloat
    + Serialize
    + for<'a> Sum<&'a Self>
    + IntegratorsFloat
{
}
impl Float for f32 {}
impl Float for f64 {}

/// The biggest floating-point type with implemented [`Float`] trait
type FloatMax = f64;

/// Run the program
#[doc(hidden)]
fn main() -> Result<()> {
    // Parse the arguments
    let args = cli::parse();
    // Create a model
    let mut model = model::Model::<f64>::from(&args).with_context(|| "Couldn't create a model")?;
    // Integrate the model
    model
        .integrate()
        .with_context(|| "Couldn't integrate the model")?;
    // Write the results
    model
        .write(&args.output)
        .with_context(|| "Couldn't write the results")?;
    Ok(())
}
