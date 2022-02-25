### Notices

Git mirrors:
- [Codeberg](https://codeberg.org/paveloom-university/Computer-Simulation-S10-2022)
- [GitHub](https://github.com/paveloom-university/Computer-Simulation-S10-2022)
- [GitLab](https://gitlab.com/paveloom-g/university/s10-2022/computer-simulation)

#### KaTeX

To build a crate's documentation with [KaTeX](https://katex.org/) support, run:

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
