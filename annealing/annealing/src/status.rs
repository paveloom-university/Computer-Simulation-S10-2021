//! Provides the [`Status`](crate::Status) enum

use num::Float;

use std::fmt::Debug;

/// Status function
pub enum Status<F: Float + Debug, const N: usize> {
    /// Don't print status
    None,
    /// Print status when `k` is divisable by `nk`
    Periodic {
        /// A number of iterations between calls
        nk: usize,
    },
    /// Custom: choose your own!
    Custom {
        /// Custom function
        f: fn(k: usize, t: F, f: F, p: [F; N], best_f: F, best_p: [F; N]),
    },
}

impl<F: Float + Debug, const N: usize> Status<F, N> {
    /// Print the status
    ///
    /// Arguments:
    /// * `k` --- Current iteration;
    /// * `t` --- Current temperature;
    /// * `f` --- Current solution;
    /// * `p` --- Current point;
    /// * `best_f` --- Current best solution;
    /// * `best_p` --- Current point of the best solution.
    pub fn print(&self, k: usize, t: F, f: F, p: [F; N], best_f: F, best_p: [F; N]) {
        match self {
            Status::None => (),
            Status::Periodic { nk } => {
                if k % nk == 0 {
                    println!(
                        "k: {k}\nt: {t:#?}:\ncurrent: {f:#?} at {p:#?}\nbest: {best_f:#?} at {best_p:#?}\n"
                    );
                }
            }
            Status::Custom { f: fun } => fun(k, t, f, p, best_f, best_p),
        }
    }
}
