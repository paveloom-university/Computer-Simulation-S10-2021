//! Provides the [`serialize_into`] function

use anyhow::{Context, Result};
use bincode::Options;
use num::Float;
use serde::Serialize;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Serialize the vector into the file
pub fn serialize_into<F: Float + Serialize>(vec: &[F], path: &Path) -> Result<()> {
    let file = File::create(path).with_context(|| "Couldn't open a file in write-only mode")?;
    let mut writer = BufWriter::new(file);

    bincode::DefaultOptions::new()
        .with_native_endian()
        .with_fixint_encoding()
        .serialize_into(&mut writer, vec)
        .with_context(|| format!("Couldn't serialize the vector for file {:?}", path))?;
    Ok(())
}
