//! Provides the command-line interface of the program

use anyhow::Result;
use clap::Parser;
use num::Float;
use numeric_literals::replace_float_literals;
use paste::paste;

use std::fmt::Debug;
use std::num::ParseFloatError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Command-line interface arguments
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args<F>
where
    F: 'static + Float + FromStr<Err = ParseFloatError> + Debug,
{
    /// Output directory
    #[clap(short, long, validator = Self::validate_output)]
    pub output: PathBuf,
    /// Maximum degree of spherical harmonics
    #[clap(short, help_heading = "MODEL", default_value_t = 4)]
    pub lmax: usize,
    /// Initial temperature
    #[clap(long = "from", help_heading = "OPTIMIZATION", default_value = "100000.0", validator = Self::validate_t_0)]
    pub t_0: F,
    /// Minimum temperature
    #[clap(long = "to", help_heading = "OPTIMIZATION", default_value = "1.0", validator = Self::validate_t_min)]
    pub t_min: F,
}

/// Create a validator for an argument
macro_rules! validator {
    ( $arg:ident, $ty:ty, $range:expr, $name:expr) => {
        paste! {
            #[doc = "Check if the " $name " is in range"]
            fn [<validate_ $arg>](s: &str) -> Result<(), String> {
                $ty::from_str(s).map(|e| $range.contains(&e))
                    .map_err(|_| format!("Couldn't parse the argument `{}`", stringify!($name)))
                    .and_then(|result| {
                        if result {
                            Ok(())
                        } else {
                            Err(format!(
                                "{} is not in the range `{:?}`",
                                stringify!($name),
                                $range
                            ))
                        }
                    })
            }
        }
    };
}

#[replace_float_literals(F::from(literal).unwrap())]
impl<F> Args<F>
where
    F: Float + FromStr<Err = ParseFloatError> + Debug,
{
    /// Check if the output directory is a valid path
    fn validate_output(s: &str) -> Result<(), String> {
        if Path::new(s).is_dir() {
            Ok(())
        } else {
            Err("output must be an existing directory".to_string())
        }
    }

    validator!(t_0, F, 0.0..F::max_value(), "initial temperature");
    validator!(t_min, F, 0.0..F::max_value(), "minimum temperature");
}

/// Parse the arguments
pub fn parse<F>() -> Args<F>
where
    F: Float + FromStr<Err = ParseFloatError> + Debug,
{
    Args::parse()
}
