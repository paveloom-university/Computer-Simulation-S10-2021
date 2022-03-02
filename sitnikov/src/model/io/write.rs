//! This module provides a routine for
//! [writing](Model#method.write) the
//! results in a directory

use anyhow::{Context, Result};
use bincode::Options;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use super::super::Model;
use crate::Float;

/// Serialize the vector into the file
fn serialize_into<F: Float>(vec: &[F], path: &Path) -> Result<()> {
    let file = File::create(path).with_context(|| "Couldn't open a file in write-only mode")?;
    let mut writer = BufWriter::new(file);

    bincode::DefaultOptions::new()
        .with_native_endian()
        .with_fixint_encoding()
        .serialize_into(&mut writer, vec)
        .with_context(|| format!("Couldn't serialize the vector for file {:?}", path))?;
    Ok(())
}

impl<F: Float> Model<F> {
    /// Serialize result vectors and write them to files in the output directory
    pub fn write(&self, output: &Path) -> Result<()> {
        serialize_into(&self.results.z, &output.join("z.bin"))
            .with_context(|| "Couldn't serialize the position vector")?;
        serialize_into(&self.results.z_v, &output.join("z_v.bin"))
            .with_context(|| "Couldn't serialize the velocity vector")?;
        Ok(())
    }
}
