//! This binary crate tests the [simulated annealing](annealing) algorithm
//! for approximating the global minimum of a given function by finding the
//! global maximum of the module of a linear combination of
//! [spherical harmonics](https://en.wikipedia.org/wiki/Spherical_harmonics)
//! in the [real form](https://en.wikipedia.org/wiki/Spherical_harmonics#Real_form):
//!
//! $$
//! f = |Y_l|, \\; \text{where} \\;\\; Y_l = \sum_{m \\, = \\, -l}^l C_m Y_{lm}, \\;\\; C_m \sim \\, U[0, 1], \\; \text{and}
//! \\\\\[2ex\]
//! Y_{lm} = \begin{cases}
//! \sqrt{2} \sqrt{\dfrac{2l + 1}{4 \pi} \dfrac{(l - |m|)!}{(l + |m|)!}} P_l^{|m|}(\cos{\theta}) \sin(|m| \varphi) & \text{if} \\; m < 0 \\\\\[2.5ex\]
//! \sqrt{\dfrac{2l + 1}{4 \pi}} P_l^m(\cos{\theta}) & \text{if} \\; m = 0 \\\\\[2.5ex\]
//! \sqrt{2} \sqrt{\dfrac{2l + 1}{4 \pi} \dfrac{(l - m)!}{(l + m)!}} P_l^m(\cos{\theta}) \cos(m \varphi) & \text{if} \\; m > 0,
//! \end{cases}
//! $$
//!
//! where $ P_l^m $ is an [associated Legendre polynomial](https://en.wikipedia.org/wiki/Associated_Legendre_polynomial).

mod cli;
mod write;

use annealing::{NeighbourMethod, Point, Schedule, Status, APF, SA};
use anyhow::{Context, Result};
use rand::prelude::*;
use rand_distr::Uniform;
use rgsl::{
    legendre::associated_polynomials::{legendre_array, legendre_array_n},
    SfLegendreNorm,
};

use std::f64::consts::{FRAC_PI_8, PI, SQRT_2};

/// Run the program
#[doc(hidden)]
#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::similar_names)]
#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    // Parse the arguments
    let args = cli::parse::<f64>();
    // Compute auxiliary variables
    let lmax = args.lmax;
    let lindex = lmax * (lmax + 1) / 2;
    let mrange = lindex..=lindex + lmax;
    let polynomials_n = legendre_array_n(lmax);
    // Prepare a random number generator
    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(1);
    // Prepare a uniform distribution
    let uni = Uniform::new(0.0, 1.0);
    // Compute the coefficients
    let mut coeffs = vec![uni.sample(&mut rng)];
    for _ in 0..=lmax {
        coeffs.push(uni.sample(&mut rng));
        coeffs.push(uni.sample(&mut rng));
    }
    // Define the objective function
    let f = move |x: &Point<f64, 2>| -> f64 {
        // Calculate all normalized associated Legendre polynomials
        let mut polynomials = vec![0.; polynomials_n];
        legendre_array(
            SfLegendreNorm::SphericalHarmonic,
            lmax,
            f64::cos(x[0]),
            &mut polynomials,
        );
        // Multiply the `m = 0` member by its coefficient
        polynomials[lindex] *= coeffs[0];
        // Compute the minus of the module of a linear combination of spherical harmonics
        -f64::abs(
            polynomials[mrange.clone()]
                .iter()
                .copied()
                .enumerate()
                .reduce(|(_, accum), (m, item)| {
                    (
                        m,
                        accum
                            + item * coeffs[2 * m] * SQRT_2 * f64::cos(m as f64 * x[1])
                            + item * coeffs[2 * m + 1] * SQRT_2 * f64::sin(m as f64 * x[1]),
                    )
                })
                .unwrap()
                .1,
        )
    };
    // Prepare arrays for tracking the optimization process
    let mut ts = Vec::<f64>::new();
    let mut ps = Vec::<Vec<f64>>::new();
    let mut fs = Vec::<f64>::new();
    let mut best_ps = Vec::<Vec<f64>>::new();
    let mut best_fs = Vec::<f64>::new();
    // Define the status function
    let mut status = Status::Custom {
        f: Box::new(
            |k: usize, t: f64, f: f64, p: [f64; 2], best_f: f64, best_p: [f64; 2]| {
                if k == 1 || k % 1000 == 0 {
                    ts.push(t);
                    ps.push(p.to_vec());
                    fs.push(-f);
                    best_ps.push(best_p.to_vec());
                    best_fs.push(-best_f);
                }
            },
        ),
    };
    // Define bounds
    let bounds = [0.0..PI, 0.0..2. * PI];
    // Find the global minimum of the objective
    // function and the corresponding point
    let (minimum, point) = SA {
        f: f.clone(),
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
        status: &mut status,
        rng: &mut rng,
    }
    .findmin();
    // Convert the minimum to a maximum
    let maximum = -minimum;
    // Print the result
    println!(
        "\nmaximum: {maximum} ({} * 2π)\npoint:   {point:?} ({:?} * 2π)\n",
        maximum / (2. * PI),
        point.map(|x| x / (2. * PI)),
    );
    // Prepare a grid
    let h = 1000;
    let theta: Vec<f64> = (0..=h)
        .map(|i| bounds[0].start + i as f64 * bounds[0].end / h as f64)
        .collect();
    let phi: Vec<f64> = (0..=h)
        .map(|i| bounds[1].start + i as f64 * bounds[1].end / h as f64)
        .collect();
    // Evaluate the objective function on the grid
    let obj: Vec<f64> = theta
        .iter()
        .copied()
        .map(|theta| {
            phi.iter()
                .copied()
                .map(|phi| -f(&[theta, phi]))
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
        .concat();
    // Relinquish the mutable borrows
    drop(status);
    // Write the results
    write::serialize_into(&[maximum], &args.output.join("maximum.bin"))
        .with_context(|| "Couldn't serialize the maximum vector")?;
    write::serialize_into(&point, &args.output.join("point.bin"))
        .with_context(|| "Couldn't serialize the point vector")?;
    write::serialize_into(&theta, &args.output.join("theta.bin"))
        .with_context(|| "Couldn't serialize the polar angle vector")?;
    write::serialize_into(&phi, &args.output.join("phi.bin"))
        .with_context(|| "Couldn't serialize the azimuthal angle vector")?;
    write::serialize_into(&obj, &args.output.join("obj.bin"))
        .with_context(|| "Couldn't serialize the objective function vector")?;
    write::serialize_into(&ts, &args.output.join("ts.bin"))
        .with_context(|| "Couldn't serialize the temperature vector")?;
    write::serialize_into(&ps.concat(), &args.output.join("ps.bin"))
        .with_context(|| "Couldn't serialize the current points vector")?;
    write::serialize_into(&fs, &args.output.join("fs.bin"))
        .with_context(|| "Couldn't serialize the current solutions vector")?;
    write::serialize_into(&best_ps.concat(), &args.output.join("best_ps.bin"))
        .with_context(|| "Couldn't serialize the best points vector")?;
    write::serialize_into(&best_fs, &args.output.join("best_fs.bin"))
        .with_context(|| "Couldn't serialize the best solutions vector")?;
    Ok(())
}

#[test]
fn test_gsl_legendre() -> Result<()> {
    use anyhow::anyhow;
    use std::f64::consts::{FRAC_1_PI, SQRT_2};

    // Prepare a test point
    let theta = 0.45;
    // Calculate all normalized associated Legendre polynomials
    let lmax = 1;
    let mut polynomials = vec![0.; legendre_array_n(lmax)];
    legendre_array(
        SfLegendreNorm::SphericalHarmonic,
        lmax,
        f64::cos(theta),
        &mut polynomials,
    );
    // Compare with table values
    let v1 = 0.5 * f64::sqrt(FRAC_1_PI);
    let v2 = polynomials[0];
    if (v1 - v2).abs() >= f64::EPSILON {
        return Err(anyhow!(
            "The value of Y_{{0,0}} is incorrect: {v1} vs. {v2}"
        ));
    }
    let v1 = f64::sqrt(FRAC_1_PI * 3. / 4.) * f64::cos(theta);
    let v2 = polynomials[1];
    if (v1 - v2).abs() >= f64::EPSILON {
        return Err(anyhow!(
            "The value of Y_{{1,0}} is incorrect: {v1} vs. {v2}"
        ));
    }
    let v1 = f64::sqrt(FRAC_1_PI * 3. / 4.) * f64::sin(theta);
    let v2 = polynomials[2] * SQRT_2;
    if (v1 - v2).abs() >= f64::EPSILON {
        return Err(anyhow!(
            "The value of Y_{{1,1}} is incorrect: {v1} vs. {v2}"
        ));
    }
    Ok(())
}
