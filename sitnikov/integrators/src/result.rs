//! Provides the [`Result`] alias and its extension trait [`ResultExt`](crate::ResultExt)

use nalgebra::{DVector, Dynamic, Matrix, VecStorage};

use crate::Float;

/// The type of the result matrix
pub type Result<F> = Matrix<F, Dynamic, Dynamic, VecStorage<F, Dynamic, Dynamic>>;

/// An extension trait for the [`Result`] type
pub trait Ext<F: Float> {
    /// Initialize a matrix with `nrows` rows and `ncols` columns
    fn new(nrows: usize, ncols: usize) -> Self;
    /// Get initial values
    fn initial_values(&self) -> Vec<F>;
    /// Set the `i`-th state of the system
    fn set_state(&mut self, i: usize, x: Vec<F>);
    /// Get the `i`-th state of the system
    fn state(&self, i: usize) -> Vec<F>;
    /// Get the `i`-th result vector
    fn result(&self, i: usize) -> Vec<F>;
}

impl<F: Float> Ext<F> for Result<F> {
    fn new(nrows: usize, ncols: usize) -> Self {
        let nrows = Dynamic::new(nrows);
        let ncols = Dynamic::new(ncols);
        Matrix::zeros_generic(nrows, ncols)
    }
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
    fn result(&self, i: usize) -> Vec<F> {
        self.row(i).into_iter().copied().collect()
    }
}
