//! Provides the `crate::SimulatedAnnealing` struct and the
//! [`optimize`](crate::SimulatedAnnealing#method.optimize) method

use num::Float;
use numeric_literals::replace_float_literals;
use rand::prelude::*;
use rand_distr::{uniform::SampleUniform, Distribution, StandardNormal, Uniform};

use std::fmt::Debug;

use crate::{Bounds, NeighbourMethod, Point, Schedule, APF};

/// Parameters of the simulated annealing
///
/// Choose the temperatures and the annealing schedule wisely:
/// this is your way of controlling how long you will have to wait.
/// Note that the minimum temperature must be reachable.
pub struct SimulatedAnnealing<'a, F, R, const N: usize>
where
    F: Float,
    StandardNormal: Distribution<F>,
    R: Rng,
{
    /// Objective function
    f: fn(&Point<F, N>) -> F,
    /// Initial point
    p_0: &'a Point<F, N>,
    /// Initial temperature
    t_0: F,
    /// Minimum temperature
    t_min: F,
    /// Bounds of the parameter space
    bounds: &'a Bounds<F, N>,
    /// Acceptance probability function
    apf: &'a APF,
    /// Method of getting a random neighbour
    neighbour: &'a NeighbourMethod<F, R, N>,
    /// Annealing schedule
    schedule: &'a Schedule<F>,
    /// Random number generator
    rng: &'a mut R,
}

impl<F, R, const N: usize> SimulatedAnnealing<'_, F, R, N>
where
    F: Float + SampleUniform + Debug,
    StandardNormal: Distribution<F>,
    R: Rng + SeedableRng,
{
    /// Find the global minimum (and the corresponding point) of the objective function
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn minimum(&mut self) -> (F, Point<F, N>) {
        // Evaluate the objective function at the initial point and
        // save the initial values as the current working solution
        let mut p = *self.p_0;
        let mut f = (self.f)(self.p_0);
        // Save the current working solution as the current best
        let mut best_p = p;
        let mut best_f = f;
        // Save the initial temperature as the current one
        let mut t = self.t_0;
        // Prepare the iterations counter
        let mut k = 1;
        // Prepare a Uniform[0, 1] distribution for the APF
        let uni = Uniform::new(0., 1.);
        // Search for the minimum of the objective function
        while t > self.t_min {
            // Get a neighbor
            let neighbour_p = self.neighbour.neighbour(&p, self.bounds, self.rng);
            // Evaluate the objective function
            let neighbour_f = (self.f)(&neighbour_p);
            // Compute the difference between the new and the current solutions
            let diff = neighbour_f - f;
            // If the new solution is the new best,
            if neighbour_f < best_f {
                // Save it as the best and the current solution
                best_p = neighbour_p;
                best_f = neighbour_f;
                p = neighbour_p;
                f = neighbour_f;
            // Otherwise, if it is accepted by the acceptance probability function,
            } else if self.apf.accept(diff, t, &uni, self.rng) {
                // Save it as the current solution
                p = neighbour_p;
                f = neighbour_f;
            }
            // Lower the temperature
            t = self.schedule.cool(k, t, self.t_0);
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
    let (m, p) = SimulatedAnnealing {
        f,
        p_0: &[2.],
        t_0: 100_000.0,
        t_min: 1.0,
        bounds: &[1.0..27.8],
        apf: &APF::Metropolis,
        neighbour: &NeighbourMethod::Normal { sd: 5. },
        schedule: &Schedule::Fast,
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
