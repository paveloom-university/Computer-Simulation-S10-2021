//! This module provides a method for computing MEGNOs

use anyhow::{Context, Result};
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

impl<F: Float> Model<F> {
    /// Compute the integrand from the MEGNO expression
    #[replace_float_literals(F::from(literal).unwrap())]
    fn integrand(&self, t: F, z: F, dis_z: F, dis_z_v: F) -> Result<F> {
        // Compute the norm of the infinitesimal displacement
        let dis_norm = (dis_z.powi(2) + dis_z_v.powi(2)).sqrt();
        // Compute the radius
        let r = self
            .radius(t)
            .with_context(|| "Couldn't compute the radius")?;
        // Compute the elements of the tangent vector
        let tan_z = dis_z * (2. * z.powi(2) - r.powi(2)) / (r.powi(2) + z.powi(2)).powf(2.5);
        let tan_z_v = dis_z_v;
        // Compute the norm of the tangent vector
        let tan_norm = (tan_z * dis_z + tan_z_v * dis_z_v) / dis_norm;
        Ok(tan_norm / dis_norm * t)
    }
    /// Compute the integral (incrementally), using the trapezoidal rule
    #[replace_float_literals(F::from(literal).unwrap())]
    fn trapezoidal(&self, i: usize, integral: F, integrand_prev: F, integrand: F) -> F {
        if i == 1 {
            integral + self.h * (integrand_prev + integrand) / 2.
        } else {
            let i = F::from(i).unwrap();
            (integral + self.h * integrand_prev / 2. / (i - 1.)) * (i - 1.) / i
                + self.h * integrand / 2. / i
        }
    }
    /// Compute the Mean Exponential Growth factors of Nearby Orbits (MEGNOs)
    #[replace_float_literals(F::from(literal).unwrap())]
    pub(super) fn compute_megnos(&mut self) -> Result<()> {
        // Prepare a random number generator
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(1);
        // Add capacity to the MEGNO vectors
        self.results.megno = Vec::<F>::with_capacity(self.n);
        self.results.mean_megno = Vec::<F>::with_capacity(self.n);
        // Save the previous results
        let z_res = self.results.z.clone();
        let z_v_res = self.results.z_v.clone();
        // Variate (displace) the initial values
        self.z_0 = variate(self.z_0, &mut rng)
            .with_context(|| "Couldn't variate the initial value of position")?;
        self.z_v_0 = variate(self.z_v_0, &mut rng)
            .with_context(|| "Couldn't variate the initial value of velocity")?;
        // Integrate the model
        self.yoshida_4th()
            .with_context(|| "Couldn't integrate the model")?;
        // Compute the difference in position
        let z_delta: Vec<F> = self
            .results
            .z
            .iter()
            .zip(z_res.iter())
            .map(|(&a, &b)| (a - b).abs())
            .collect();
        // Compute the difference in velocity
        let z_v_delta: Vec<F> = self
            .results
            .z_v
            .iter()
            .zip(z_v_res.iter())
            .map(|(&a, &b)| (a - b).abs())
            .collect();
        // Put initial values to integrals
        let mut megno_integral = 0.;
        let mut mean_megno_integral = 0.;
        // Put initial values to previous evaluations of the integrands
        let mut integrand_prev = 0.;
        // Compute
        for i in 1..=self.n {
            // Compute the time moment
            let t = self.t_0 + F::from(i).unwrap() * self.h;
            // Get the result value of position
            let z = z_res[i];
            // Get the delta values of position and velocity
            let dis_z = z_delta[i];
            let dis_z_v = z_v_delta[i];
            // Compute the new integrand
            let integrand = self
                .integrand(t, z, dis_z, dis_z_v)
                .with_context(|| "Couldn't compute the intergand")?;
            // Compute the integral for MEGNO, using the trapezoidal rule
            megno_integral = self.trapezoidal(i, megno_integral, integrand_prev, integrand);
            // Compute the MEGNO
            let megno = 2. / t * megno_integral;
            self.results.megno.push(megno);
            // Compute the integral for mean MEGNO, using the trapezoidal rule
            mean_megno_integral =
                self.trapezoidal(i, mean_megno_integral, self.results.megno[i - 1], megno);
            // Compute the mean MEGNO
            let mean_megno = 1. / t * mean_megno_integral;
            self.results.mean_megno.push(mean_megno);
            // Update the previous values
            integrand_prev = integrand;
        }
        // Return the result vectors
        self.results.z = z_res;
        self.results.z_v = z_v_res;
        Ok(())
    }

    // /// Compute the Mean Exponential Growth factors of Nearby Orbits (MEGNOs)
    // #[replace_float_literals(F::from(literal).unwrap())]
    // pub(super) fn compute_megnos(&mut self) -> Result<()> {
    //     // Prepare a random number generator
    //     let mut rng = Xoshiro256PlusPlus::seed_from_u64(1);
    //     // Add capacity to the MEGNO vectors
    //     self.results.megno = Vec::<F>::with_capacity(self.n);
    //     self.results.mean_megno = Vec::<F>::with_capacity(self.n);
    //     // Put initial values to integrals
    //     let mut megno_integral = 0.;
    //     let mut mean_megno_integral = 0.;
    //     // Variate (displace) the initial values
    //     let dis_z = variate(self.z_0, &mut rng)?;
    //     let dis_z_v = variate(self.z_v_0, &mut rng)?;
    //     // Put initial values to previous evaluations of the integrands
    //     let mut integrand_prev = self
    //         .integrand(0., self.z_0, dis_z, dis_z_v)
    //         .with_context(|| "Couldn't compute the integrand")?;
    //     // Compute
    //     for i in 1..=self.n {
    //         // Compute the time moment
    //         let t = self.t_0 + F::from(i).unwrap() * self.h;
    //         // Get the current values of position and velocity
    //         let z = self.results.z[i];
    //         let z_v = self.results.z_v[i];
    //         // Variate (displace) the new pair
    //         let dis_z = variate(z, &mut rng)?;
    //         let dis_z_v = variate(z_v, &mut rng)?;
    //         // Compute the new integrand
    //         let integrand = self
    //             .integrand(t, z, dis_z, dis_z_v)
    //             .with_context(|| "Couldn't compute the intergand")?;
    //         // Compute the integral for MEGNO, using the trapezoidal rule
    //         megno_integral = self.trapezoidal(i, megno_integral, integrand_prev, integrand);
    //         // Compute the MEGNO
    //         let megno = 2. / t * megno_integral;
    //         self.results.megno.push(megno);
    //         // Compute the integral for mean MEGNO, using the trapezoidal rule
    //         mean_megno_integral =
    //             self.trapezoidal(i, mean_megno_integral, self.results.megno[i - 1], megno);
    //         // Compute the mean MEGNO
    //         let mean_megno = 1. / t * mean_megno_integral;
    //         self.results.mean_megno.push(mean_megno);
    //         // Update the previous values
    //         integrand_prev = integrand;
    //     }
    //     Ok(())
    // }
}
