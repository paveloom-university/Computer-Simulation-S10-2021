//! Provides the [`APF`](crate::APF) enum

use num::Float;
use numeric_literals::replace_float_literals;
use rand::prelude::*;
use rand_distr::{uniform::SampleUniform, Uniform};

/// Acceptance probability function
pub enum APF {
    /// Metropolis criterion:
    ///
    /// $
    /// P(\Delta f, t) = \begin{cases}
    /// 1, & if \\; \Delta f \leqslant 0; \\\\
    /// \min(e^{- \Delta f / t}, 1), & if \\; \Delta f \gt 0
    /// \end{cases}
    /// $
    Metropolis,
}

impl APF {
    /// Choose whether to accept the point
    ///
    /// Arguments:
    /// * `diff` --- Difference in the objective;
    /// * `t` --- Temperature;
    /// * `uni` -- Uniform[0, 1] distribution;
    /// * `rng` --- Random number generator.
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn accept<F, R>(&self, diff: F, t: F, uni: &Uniform<F>, rng: &mut R) -> bool
    where
        F: Float + SampleUniform,
        R: Rng,
    {
        match self {
            APF::Metropolis => diff < 0. || uni.sample(rng) < F::min(F::exp(-diff / t), 1.),
        }
    }
}
