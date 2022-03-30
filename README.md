### Notices

#### Mirrors

Repository:
- [Codeberg](https://codeberg.org/paveloom-university/Computer-Simulation-S10-2022)
- [GitHub](https://github.com/paveloom-university/Computer-Simulation-S10-2022)
- [GitLab](https://gitlab.com/paveloom-g/university/s10-2022/computer-simulation)

References:
- [sitnikov](sitnikov) [[GH](https://paveloom-university.github.io/Computer-Simulation-S10-2022)] [[GL](https://paveloom-g.gitlab.io/university/s10-2022/computer-simulation)]: Simulating the Sitnikov problem
  - [integrators](sitnikov/integrators) [[GH](https://paveloom-university.github.io/Computer-Simulation-S10-2022/sitnikov/integrators/index.html)] [[GL](https://paveloom-g.gitlab.io/university/s10-2022/computer-simulation/sitnikov/integrators/index.html)]: Providing an interface for integrating a system of 1st-order ODEs

#### Rust

This project provides [Rust](https://www.rust-lang.org) crates. To build them, use [Cargo](https://doc.rust-lang.org/cargo).

#### CLI

This project provides binaries with command-line interfaces. To see the usage examples, run `cargo
run -- --help`.

#### Tests

To run tests, consider using [`nextest`](https://nexte.st).

#### KaTeX

To build a crate's documentation with [KaTeX](https://katex.org) support, run:

```bash
cargo doc
RUSTDOCFLAGS="--html-in-header assets/katex-header.html" cargo doc --no-deps --open
```

#### Julia

This project provides [Julia](https://julialang.org) scripts. Make sure to use the project files (`Project.toml`) when running them:

```bash
julia --project=. -e "using Pkg; Pkg.instantiate()"
julia --project=. scripts/script.jl
```

Alternatively, you can use the `julia.bash` script, which starts a [daemon](https://github.com/dmolina/DaemonMode.jl) and runs scripts through it:

```bash
julia --project=. -e "using Pkg; Pkg.instantiate()"
./julia.bash scripts/script.jl
```

To kill the daemon run

```bash
./julia.bash kill
```

#### Pluto

This project provides Pluto notebooks. You can interact with them in the web interface:

```bash
julia --project=. -e "using Pkg; Pkg.instantiate()"
julia --project=.
```

```julia
using Pluto
Pluto.run()
```

Alternatively, you can run them as scripts:

```
julia --project=. -e "using Pkg; Pkg.instantiate()"
julia --project=. notebooks/pluto/notebook.jl
```
