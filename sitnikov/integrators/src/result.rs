//! Provides the [`Result`] alias and its extension trait [`ResultExt`]

use nalgebra::{DVector, Dynamic, Matrix, VecStorage};

use crate::Float;

/// The type of the result matrix
pub type Result<F> = Matrix<F, Dynamic, Dynamic, VecStorage<F, Dynamic, Dynamic>>;

/// An extension trait for the [`Result`] type
#[allow(clippy::module_name_repetitions)]
pub trait ResultExt<F: Float> {
    /// Get initial values
    fn initial_values(&self) -> Vec<F>;
    /// Set the `i`-th state of the system
    fn set_state(&mut self, i: usize, x: Vec<F>);
    /// Get the `i`-th state of the system
    fn state(&self, i: usize) -> Vec<F>;
}

impl<F: Float> ResultExt<F> for Result<F> {
    fn initial_values(&self) -> Vec<F> {
        self.state(0)
    }
    fn set_state(&mut self, i: usize, x: Vec<F>) {
        let x = DVector::from(x);
        self.set_column(i, &x);
    }
    fn state(&self, i: usize) -> Vec<F> {
        self.column(i).into_iter().copied().collect()
    }
}
