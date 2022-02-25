//! This module defines the command-line interface of the program

use anyhow::Result;
use clap::Parser;
use paste::paste;

use std::ops::{Range, RangeInclusive};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::{F, I};

/// Only circular (0) and elliptical (0 < e < 1) orbits are considered
const E_RANGE: Range<F> = 0.0..1.0;

/// Time step is bound by the machine epsilon value
const T_RANGE: RangeInclusive<F> = F::EPSILON..=1e-1;

/// Number of iterations is bound by the largest representable value
const N_RANGE: RangeInclusive<I> = 1..=I::MAX;

/// Initial value of position of the third body is bound by the largest representable value
const Z_0_RANGE: RangeInclusive<F> = -F::MAX..=F::MAX;

/// Initial value of velocity of the third body is bound by the largest representable value
const Z_V_0_RANGE: RangeInclusive<F> = -F::MAX..=F::MAX;

/// Command-line interface arguments
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Output directory
    #[clap(short, long, validator = validate_output)]
    pub output: PathBuf,
    /// Eccentricity
    #[clap(short, help_heading = "MODEL", default_value_t = 0., validator = validate_e)]
    pub e: F,
    /// Initial value of position of the third body
    #[clap(short = 'p', help_heading = "MODEL", default_value_t = 1., validator = validate_z_0)]
    pub z_0: F,
    /// Initial value of velocity of the third body
    #[clap(short = 'v', help_heading = "MODEL", default_value_t = 0., validator = validate_z_v_0)]
    pub z_v_0: F,
    /// Time step
    #[clap(short, help_heading = "INTEGRATION", default_value_t = 1e-2, validator = validate_t)]
    pub h: F,
    /// Number of iterations
    #[clap(short, help_heading = "INTEGRATION", default_value_t = 100000, validator = validate_n)]
    pub n: I,
}

/// Parse the arguments
pub fn parse() -> Args {
    Args::parse()
}

/// Check if the output directory is a valid path
fn validate_output(s: &str) -> Result<(), String> {
    if Path::new(s).is_dir() {
        Ok(())
    } else {
        Err("output must be an existing directory".to_string())
    }
}

/// Create a validator for an argument
macro_rules! validator {
    ( $arg:ident, $ty:ty ) => {
        validator!($arg, $ty, $arg);
    };
    ( $arg:ident, $ty:ty, $name:expr) => {
        paste! {
            #[doc = "Check if the " $name " is in range"]
            fn [<validate_ $arg>](s: &str) -> Result<(), String> {
                $ty::from_str(s).map(|e| [<$arg:upper _RANGE>].contains(&e))
                    .map_err(|_| format!("Couldn't parse the argument `{}`", stringify!($name)))
                    .and_then(|result| {
                        if result {
                            Ok(())
                        } else {
                            Err(format!(
                                "{} is not in the range `{:?}`",
                                stringify!($name),
                                [<$arg:upper _RANGE>]
                            ))
                        }
                    })
            }
        }
    };
}

validator!(e, F, "eccentricity");
validator!(z_0, F, "initial value of position of the third body");
validator!(z_v_0, F, "initial value of velocity of the third body");
validator!(t, F, "time step");
validator!(n, I, "number of iterations");
