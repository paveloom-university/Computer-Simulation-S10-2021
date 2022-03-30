//! Provides the [`integrate`](Model#method.integrate) method

use anyhow::{Context, Result};
use integrators::{
    GeneralIntegrator, GeneralIntegrators, ResultExt, SymplecticIntegrator, SymplecticIntegrators,
};
use numeric_literals::replace_float_literals;
use rand::prelude::*;
use rand_distr::Normal;
use rand_xoshiro::Xoshiro256PlusPlus;

use super::super::Model;
use crate::{Float, FloatMax};

/// Get a small variation to the passed value
fn variate<F: Float>(x: F, rng: &mut impl rand::Rng) -> Result<F> {
    // Construct a normal distribution with the passed value as mean
    let normal: Normal<FloatMax> = Normal::new(x.to_f64().unwrap(), 1e-1)
        .with_context(|| "Couldn't construct a normal distribution for {x}")?;
    // Sample a number from this distribution
    Ok(F::from(normal.sample(rng)).unwrap())
}

impl<F: Float> SymplecticIntegrator<F> for Model<F> {
    // We integrate the equations of motion for one or two trajectories
    fn accelerations(&self, t: F, x: &[F]) -> Result<Vec<F>> {
        // Compute the acceleration
        let a = self
            .acceleration(t, x[0])
            .with_context(|| "Couldn't compute the acceleration")?;
        if self.compute_megnos {
            // Compute the acceleration of the second trajectory
            let a_tilda = self
                .acceleration(t, x[1])
                .with_context(|| "Couldn't compute the acceleration of the second trajectory")?;
            Ok(vec![a, a_tilda])
        } else {
            Ok(vec![a])
        }
    }
}

impl<F: Float> GeneralIntegrator<F> for Model<F> {
    // We integrate the equations of motions and MEGNO equations
    #[replace_float_literals(F::from(literal).unwrap())]
    fn update(&self, t: F, x: &[F]) -> Result<Vec<F>> {
        // Compute the accelerations
        let a_1 = self
            .acceleration(t, x[0])
            .with_context(|| "Couldn't compute the acceleration of the first trajectory")?;
        let a_2 = self
            .acceleration(t, x[1])
            .with_context(|| "Couldn't compute the acceleration of the second trajectory")?;
        // Compute the deltas
        let delta_z = x[1] - x[0];
        let delta_z_v = x[3] - x[2];
        let delta_a = a_2 - a_1;
        // Compute the scalars
        let delta_dot_pr = delta_z_v * delta_z + delta_a * delta_z_v;
        let delta_norm_sq = delta_z.powi(2) + delta_z_v.powi(2);
        // Return the new state
        Ok(vec![
            x[2],
            x[3],
            a_1,
            a_2,
            // The following two equations compute the integrands from the equations
            // for MEGNOs and mean MEGNOs (see T. C. Hinse et al., 2010). Note that,
            // technically, these should have `t` - `t_0` instead of `t`, because
            // both equations come from the formulae that represent the "mean
            // exponential rate of divergence of nearby orbits". However, there
            // is a singular point at `t - t_0 = 0`. Since the properties at
            // t -> +Inf are the same for `t`, substitution of`t_0` is omitted
            delta_dot_pr / delta_norm_sq * t,
            2. * x[4] / t,
        ])
    }
}

impl<F: Float> Model<F> {
    /// Integrate the equations of motion and
    /// (optionally) compute MEGNOs
    #[replace_float_literals(F::from(literal).unwrap())]
    pub(crate) fn integrate(&mut self) -> Result<()> {
        // If a user wants to compute MEGNOs
        if self.compute_megnos {
            // Prepare a random number generator
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(1);
            // Variate (displace) the initial values
            let z_0_tilda = variate(self.x_0[0], &mut rng)
                .with_context(|| "Couldn't variate the initial value of position")?;
            let z_v_0_tilda = variate(self.x_0[1], &mut rng)
                .with_context(|| "Couldn't variate the initial value of velocity")?;
            // Compute the initial acceleration for the displaced value of position
            let a_0_tilda = self.acceleration(self.t_0, z_0_tilda).with_context(|| {
                "Couldn't compute the initial acceleration with displaced initial position"
            })?;
            // Integrate the equations of motion
            // using the 4th-order Yoshida method
            // (`i_m` iterations)
            //
            // This is because we'd like to avoid the singular
            // point at `t = 0` when computing MEGNOs later
            self.results.x = SymplecticIntegrator::integrate(
                self,
                &[
                    self.x_0[0],
                    z_0_tilda,
                    self.x_0[1],
                    z_v_0_tilda,
                    self.x_0[2],
                    a_0_tilda,
                ],
                self.t_0,
                self.h,
                self.i_m,
                SymplecticIntegrators::Yoshida4th,
            )
            .with_context(|| "Couldn't integrate the equations of motion")?;
            // Get the `i_m`-th state of the system of the equation of motions
            let s = self.results.x.state(self.i_m);
            // Compute the time moment
            let t_0 = self.t_0 + F::from(self.i_m).unwrap() * self.h;
            // Compute the next number of iterations
            let n_m = self.n - self.i_m;
            // Compute the integrals in the MEGNO equations
            // using the 4th-order Runge-Kutta method
            // (`n` - `i_m` iterations)
            self.results.m = GeneralIntegrator::integrate(
                self,
                &[s[0], s[1], s[2], s[3], 0., 0.],
                t_0,
                self.h,
                n_m,
                GeneralIntegrators::RungeKutta4th,
            )
            .with_context(|| "Couldn't integrate the MEGNO equations")?;
            // Compute the MEGNOs
            for i in 0..=n_m {
                // Compute the time moment
                let t = t_0 + F::from(i + self.i_m).unwrap() * self.h;
                // Compute the MEGNO (see the note about `t` above)
                self.results.m[(4, i)] = 2. * self.results.m[(4, i)] / t;
                // Compute the mean MEGNO (see the note about `t` above)
                self.results.m[(5, i)] = self.results.m[(5, i)] / t;
            }
            // Otherwise,
        } else {
            // Integrate the equations of motion
            // using the 4th-order Yoshida method
            self.results.x = SymplecticIntegrator::integrate(
                self,
                &self.x_0,
                self.t_0,
                self.h,
                self.n,
                SymplecticIntegrators::Yoshida4th,
            )
            .with_context(|| "Couldn't integrate the equations of motion")?;
        }
        Ok(())
    }
}
