#!/bin/bash

# A script to document the crates

# Create an output directory
mkdir -p public

# Read the names of the crates in an array
mapfile -t crates < <(find . -maxdepth 1 -mindepth 1 -not -path '*/.*' -not -name 'public' -type d -printf '%f\n');

echo

# For each crate
for crate in "${crates[@]}"; do
    # Go to its root directory
    cd "${crate}" || continue
    # Document the crate
    cargo doc --workspace --document-private-items
    RUSTDOCFLAGS="--html-in-header assets/katex-header.html" cargo doc --no-deps --workspace --document-private-items
    # Copy the documentation to the output directory
    cp -r target/doc/ ../public/"${crate}"
    echo
    # Get back to the repository's root
    cd - >/dev/null || continue
done

# Redirect to the first crate's reference
echo "<meta http-equiv=\"refresh\" content=\"0; url=${crates[0]}/${crates[0]}\">" >public/index.html
