//! This module defines the command-line interface of the program

use anyhow::Result;
use clap::Parser;
use numeric_literals::replace_float_literals;
use paste::paste;

use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::Float;

/// Command-line interface arguments
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args<F: 'static + Float> {
    /// Output directory
    #[clap(short, long, validator = Self::validate_output)]
    pub output: PathBuf,
    /// Compute MEGNOs?
    #[clap(long = "megno")]
    pub compute_megnos: bool,
    /// Eccentricity
    #[clap(short, help_heading = "MODEL", default_value = "0.0", validator = Self::validate_e)]
    pub e: F,
    /// Time at the pericenter (a fraction of $ 2 \pi $)
    #[clap(short, help_heading = "MODEL", default_value = "0.0", validator = Self::validate_tau)]
    pub tau: F,
    /// Initial value of time
    #[clap(short = 'a', help_heading = "MODEL", default_value = "1.0", validator = Self::validate_t_0)]
    pub t_0: F,
    /// Initial value of position of the third body
    #[clap(short = 'p', help_heading = "MODEL", default_value = "1.0", validator = Self::validate_z_0)]
    pub z_0: F,
    /// Initial value of velocity of the third body
    #[clap(short = 'v', help_heading = "MODEL", default_value = "0.0", validator = Self::validate_z_v_0)]
    pub z_v_0: F,
    /// Time step (multiple of $ \pi / 2 $)
    #[clap(short, help_heading = "INTEGRATION", default_value = "1e-2", validator = Self::validate_h)]
    pub h: F,
    /// Number of periods (multiple of $ 2 \pi $)
    #[clap(short = 'P', help_heading = "INTEGRATION", default_value_t = 1000, validator = Self::validate_p)]
    pub p: usize,
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
impl<F: 'static + Float> Args<F> {
    /// Check if the output directory is a valid path
    fn validate_output(s: &str) -> Result<(), String> {
        if Path::new(s).is_dir() {
            Ok(())
        } else {
            Err("output must be an existing directory".to_string())
        }
    }

    /// Check if the time step is valid
    fn validate_h(s: &str) -> Result<(), String> {
        let range = F::epsilon()..=1e-1;
        F::from_str(s)
            .map_err(|_| "Couldn't parse the argument `h`".to_string())
            .and_then(|h| {
                if range.contains(&h) {
                    let a = 4. / h;
                    let b = (4. / h).round();
                    if (a - b).abs() < F::epsilon() {
                        Ok(())
                    } else {
                        Err("time step is incorrect; ".to_string()
                            + "make sure that the expression `4 / h` gives an integral value")
                    }
                } else {
                    Err(format!("time step is not in the range `{:?}`", range))
                }
            })
    }

    validator!(e, F, 0.0..1.0, "eccentricity");
    validator!(tau, F, 0.0..1.0, "time at the pericenter");
    validator!(
        t_0,
        F,
        -F::max_value()..=F::max_value(),
        "initial value of time"
    );
    validator!(
        z_0,
        F,
        -F::max_value()..=F::max_value(),
        "initial value of position of the third body"
    );
    validator!(
        z_v_0,
        F,
        -F::max_value()..=F::max_value(),
        "initial value of velocity of the third body"
    );
    validator!(p, usize, 1..=usize::MAX, "number of periods");
}

/// Parse the arguments
pub fn parse<F: Float>() -> Args<F> {
    Args::parse()
}
