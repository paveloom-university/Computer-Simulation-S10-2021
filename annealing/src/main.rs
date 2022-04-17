//! This binary crate tests the [simulated annealing](annealing) algorithm
//! for approximating the global minimum of a given function by finding the
//! global maximum of the module of a
//! [spherical harmonic](https://en.wikipedia.org/wiki/Spherical_harmonics)
//! in the [real form](https://en.wikipedia.org/wiki/Spherical_harmonics#Real_form).

use annealing::{NeighbourMethod, Point, Schedule, Status, APF, SA};
use rand::prelude::*;
use rgsl::{
    legendre::associated_polynomials::{legendre_array, legendre_array_n},
    SfLegendreNorm,
};

use std::f64::consts::{FRAC_PI_8, PI, SQRT_2};

mod cli;

/// Run the program
#[doc(hidden)]
fn main() {
    // Parse the arguments
    let args = cli::parse::<f64>();
    // Compute auxiliary variables
    let lmax = args.lmax;
    let lindex = lmax * (lmax + 1) / 2;
    let mrange = lindex..=lindex + lmax;
    // Define the objective function
    #[allow(clippy::cast_precision_loss)]
    let f = move |x: &Point<f64, 2>| -> f64 {
        // Calculate all normalized associated Legendre polynomials
        let mut polynomials = vec![0.; legendre_array_n(lmax)];
        legendre_array(
            SfLegendreNorm::SphericalHarmonic,
            lmax,
            f64::cos(x[0]),
            &mut polynomials,
        );
        // Compute the minus of the module of the spherical harmonic
        -f64::abs(
            polynomials[mrange.clone()]
                .iter()
                .copied()
                .enumerate()
                .reduce(|(_, accum), (m, item)| {
                    (m, accum + item * SQRT_2 * f64::cos(m as f64 * x[1]))
                })
                .unwrap()
                .1,
        )
    };
    // Prepare a random number generator
    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(1);
    // Define bounds
    let bounds = [0.0..PI, 0.0..2. * PI];
    // Find the global minimum of the objective
    // function and the corresponding point
    let (minimum, point) = SA {
        f,
        p_0: &[
            rng.gen_range(bounds[0].clone()),
            rng.gen_range(bounds[1].clone()),
        ],
        t_0: args.t_0,
        t_min: args.t_min,
        bounds: &bounds,
        apf: &APF::Metropolis,
        neighbour: &NeighbourMethod::Normal { sd: FRAC_PI_8 },
        schedule: &Schedule::Fast,
        status: &Status::None,
        rng: &mut rng,
    }
    .findmin();
    // Convert the minimum to a maximum
    let maximum = -minimum;
    // Print the result
    println!("\nmaximum: {maximum}\npoint:   {point:?}\n");
}
