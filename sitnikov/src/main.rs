//! This binary crate is a command-line utility for simulating the
//! [Sitnikov problem](https://en.wikipedia.org/wiki/Sitnikov_problem).

mod cli;
mod model;

use anyhow::{Context, Result};

/// Some of the basic mathematical constants
mod consts {
    pub use std::f64::consts::PI;
}

/// The floating point type used across the program
type F = f64;

/// The integer type used across the program
type I = u32;

/// Run the program
fn main() -> Result<()> {
    // Parse the arguments
    let args = cli::parse();
    // Create a model
    let mut model = model::Model::from(&args);
    // Integrate the equations of motion
    model
        .integrate()
        .with_context(|| "Couldn't integrate the model")?;
    // Write the results
    model
        .write(&args.output)
        .with_context(|| "Couldn't write the results")?;
    Ok(())
}
