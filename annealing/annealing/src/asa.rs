//! Provides the [`ASA`](crate::ASA) struct and the
//! [`minimum`](crate::ASA#method.minimum) method

use num::Float;
use numeric_literals::replace_float_literals;
use rand::prelude::*;
use rand_distr::{uniform::SampleUniform, Distribution, StandardNormal, Uniform};

use std::fmt::Debug;

use crate::{Bounds, Point, Schedule, APF};

/// Adaptive simulated annealing
pub struct ASA<'a, F, R, const N: usize>
where
    F: Float + SampleUniform,
    StandardNormal: Distribution<F>,
    R: Rng,
{
    /// Objective function
    f: fn(&Point<F, N>) -> F,
    /// Initial point
    p_0: &'a Point<F, N>,
    /// Initial step vector
    h_0: &'a Point<F, N>,
    /// Step multiplication factors
    c: &'a Point<F, N>,
    /// Initial temperature
    t_0: F,
    /// Minimum temperature
    t_min: F,
    /// Bounds of the parameter space
    bounds: &'a Bounds<F, N>,
    /// Acceptance probability function
    apf: &'a APF<F, R>,
    /// Annealing schedule
    schedule: &'a Schedule<F>,
    /// Number of cycles of random moves
    nm: usize,
    /// Number of step vector adjustments
    na: usize,
    /// Random number generator
    rng: &'a mut R,
}

impl<F, R, const N: usize> ASA<'_, F, R, N>
where
    F: Float + SampleUniform + Debug,
    StandardNormal: Distribution<F>,
    R: Rng + SeedableRng,
{
    /// Find the global minimum (and the corresponding point) of the objective function
    #[allow(clippy::many_single_char_names)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn minimum(&mut self) -> (F, Point<F, N>) {
        // Evaluate the objective function at the initial point and
        // save the initial values as the current working solution
        let mut p = *self.p_0;
        let mut f = (self.f)(self.p_0);
        // Save the current working solution as the current best
        let mut best_p = p;
        let mut best_f = f;
        // Save the initial step vector as the current one
        let mut h = *self.h_0;
        // Save the initial temperature as the current one
        let mut t = self.t_0;
        // Prepare the iterations counter
        let mut k = 1;
        // Prepare a Uniform[-1, 1] distribution for the method of getting neighbours
        let neighbour_uni = Uniform::new(-1., 1.);
        // Prepare a Uniform[0, 1] distribution for the APF
        let apf_uni = Uniform::new(0., 1.);
        // Prepare an array of the numbers of accepted points
        let mut a: [usize; N] = [0; N];
        // Convert the number of cycles of random moves to a floating-point type
        let nm_f = F::from(self.nm).unwrap();
        // Search for the minimum of the objective function
        while t > self.t_min {
            // Do a cycle of step adjustments
            for _ in 0..self.na {
                // Do a cycle of random moves
                for _ in 0..self.nm {
                    // For each coordinate of the current point
                    for i in 0..N {
                        // Get a neighbour by modifying the coordinate
                        let mut neighbour_p = p;
                        let mut coordinate = p[i] + neighbour_uni.sample(self.rng) * h[i];
                        // If the result is not in the range, repeat until it is
                        while !self.bounds[i].contains(&coordinate) {
                            coordinate = p[i] + neighbour_uni.sample(self.rng) * h[i];
                        }
                        // Update the coordinate
                        neighbour_p[i] = coordinate;
                        // Evaluate the objective function
                        let neighbour_f = (self.f)(&neighbour_p);
                        // Compute the difference between the new and the current solutions
                        let diff = neighbour_f - f;
                        // If the new solution is accepted by the acceptance probability function,
                        if self.apf.accept(diff, t, &apf_uni, self.rng) {
                            // Save it as the current solution
                            p = neighbour_p;
                            f = neighbour_f;
                            // Update the counter of accepted points
                            a[i] += 1;
                        }
                        // If the new solution is the new best,
                        if neighbour_f < best_f {
                            // Save it as the new best
                            best_p = neighbour_p;
                            best_f = neighbour_f;
                        }
                    }
                }
                // Adjust the step vector
                for i in 0..N {
                    let ai = F::from(a[i]).unwrap();
                    if ai > 0.6 * nm_f {
                        h[i] = h[i] * (1. + self.c[i] * (ai / nm_f - 0.6) / 0.4);
                    } else if ai < 0.4 * nm_f {
                        h[i] = h[i] / (1. + self.c[i] * (0.4 - ai / nm_f) / 0.4);
                    }
                }
                // Reset the counters of accepted points
                a = [0; N];
            }
            // Lower the temperature
            t = self.schedule.cool(k, t, self.t_0);
            dbg!(t);
            // Update the iterations counter
            k += 1;
        }
        (best_f, best_p)
    }
}

#[cfg(test)]
use anyhow::{anyhow, Result};

#[test]
fn test() -> Result<()> {
    // Define the objective function
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn f(p: &Point<f64, 1>) -> f64 {
        let x = p[0];
        f64::ln(x) * (f64::sin(x) + f64::cos(x))
    }
    // Get the minimum
    let (m, p) = ASA {
        f,
        p_0: &[2.],
        h_0: &[0.25],
        c: &[1.],
        t_0: 20.0,
        t_min: 1.0,
        bounds: &[1.0..27.8],
        apf: &APF::Metropolis,
        schedule: &Schedule::Exponential { gamma: 0.75 },
        nm: 20,
        na: 10,
        rng: &mut rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(1),
    }
    .minimum();
    // Compare the result with the actual minimum
    let actual_p = [22.790_580_66];
    let actual_m = f(&actual_p);
    if (p[0] - actual_p[0]).abs() >= 1e-4 {
        return Err(anyhow!(
            "The minimum point is incorrect: {} vs. {}",
            actual_p[0],
            p[0]
        ));
    }
    if (m - actual_m).abs() >= 1e-9 {
        return Err(anyhow!(
            "The minimum value is incorrect: {} vs. {}",
            actual_m,
            m
        ));
    }
    Ok(())
}
