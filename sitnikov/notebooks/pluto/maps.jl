### A Pluto.jl notebook ###
# v0.18.1

using Markdown
using InteractiveUtils

# This Pluto notebook uses @bind for interactivity. When running this notebook outside of Pluto, the following 'mock version' of @bind gives bound variables a default value (instead of an error).
macro bind(def, element)
    quote
        local iv = try Base.loaded_modules[Base.PkgId(Base.UUID("6e696c72-6542-2067-7265-42206c756150"), "AbstractPlutoDingetjes")].Bonds.initial_value catch; b -> missing; end
        local el = $(esc(element))
        global $(esc(def)) = Core.applicable(Base.get, el) ? Base.get(el) : iv(el)
        el
    end
end

# ╔═╡ 6a87a898-9dfd-11ec-2b38-49b883798d79
begin
CURRENT_DIR = @__DIR__
ROOT_DIR = if basename(CURRENT_DIR) == "pluto"
    dirname(dirname(CURRENT_DIR))
else
    CURRENT_DIR
end

# Activate the environment
using Pkg
Pkg.activate(ROOT_DIR; io = devnull)

# Disable warnings
using Logging
disable_logging(Logging.Warn)

println('\n', " "^4, "> Loading the packages...")

using LaTeXStrings
using Plots
using PlutoUI

# Use the GR backend for plots
gr()

# Change some of the default parameters for plots
default(fontfamily = "Computer Modern", dpi = 300, legend = nothing)

# Define the paths to output directories
DATA_DIR = joinpath(ROOT_DIR, "data", "notebooks", "maps")
PLOTS_DIR = joinpath(ROOT_DIR, "plots", "notebooks", "maps")

# Make sure the needed directories exist
mkpath(DATA_DIR)
mkpath(PLOTS_DIR)

# Define the floating point type used across the script
F = Float64

# Define the integer type used across the script
I = UInt64

md"This is a Pluto notebook for plotting the Poincaré (stroboscopic) maps."
end

# ╔═╡ 4253f362-f966-45af-ac9c-0016672f5952
begin
# A step for the sliders
S_STEP = 0.001

# The maximum floating point type number
# that is reachable with the specified step
F_MAX = floor(typemax(Int) - 1000) * S_STEP / 2

# Eccentricity
E_S = @bind E NumberField(
    0.0:S_STEP:0.999,
    default = 0.0,
)

# Time step
H_S = @bind H NumberField(
    S_STEP:S_STEP:0.1,
    default = 0.01,
)

# Number of periods
P_S = @bind P NumberField(
    1:1:10000,
    default = 1000,
)

md"""
Parameters of the model:\
``e`` $(E_S) ``h \; [\pi / 2]``: $(H_S) ``P``: $(P_S)
"""
end

# ╔═╡ 4fe5044e-4f67-4bb5-afe9-f5f95ac02c78
begin

# Define a tuple of pairs of initial values
INITIAL_VALUES = ((i, 0.) for i in 0.2:0.1:1.8)

"Read binary files in the `bincode` format"
function read_bincode(path::AbstractString)::Tuple{I, Vector{F}}
    open(path, "r") do io
        n = read(io, I)
        a = reinterpret(F, read(io, sizeof(F) * n))
        n, a
    end
end

# Prepare an empty plot
p = scatter(size = (400, 400));

# For each pair of initial values
for pair in INITIAL_VALUES
    # Define the path to the data file
    data_problem_dir = joinpath(DATA_DIR, "$pair")
    plots_problem_dir = joinpath(PLOTS_DIR, "$pair")
    # Make sure the needed directories exist
    mkpath(data_problem_dir)
    mkpath(plots_problem_dir)
    # Define the arguments
    args = [
        "-e", E,
        "-h", H,
        "-P", P,
        "-p", pair[begin],
        "-v", pair[end],
        "-o", data_problem_dir
    ]
    # Integrate the problem
    run(pipeline(`cargo run -r -- $args`, stderr = devnull))
    # Define the paths to the binary files
    z_path = joinpath(data_problem_dir, "z.bin")
    z_v_path = joinpath(data_problem_dir, "z_v.bin")
    # Read the data
    n, z = read_bincode(z_path)
    _, z_v = read_bincode(z_v_path)
    # Compute the number of points per period
    np = UInt((n - 1) / P)
    # Plot the figure
    println(" "^4, "> Plotting the Poincaré map for the $pair pair...")
    s = scatter(
        z[1:np:end],
        z_v[1:np:end];
        label = "",
        title = "Poincaré map",
        xlabel = L"z",
        ylabel = L"\dot{z}",
        size = (400, 400),
        markersize = 0.5,
    );
    # Save the figure as PDF and PNG
    savefig(s, joinpath(plots_problem_dir, "Poincaré map.pdf"));
    savefig(s, joinpath(plots_problem_dir, "Poincaré map.png"));
    # Plot the data on the gerenal plot
    scatter!(
        p,
        z[1:np:end],
        z_v[1:np:end];
        label = "",
        title = "Poincaré map",
        xlabel = L"z",
        ylabel = L"\dot{z}",
        size = (400, 400),
        markersize = 0.5,
    );
end

# Save the final figure as PDF and PNG
savefig(p, joinpath(PLOTS_DIR, "Poincaré map.pdf"));
savefig(p, joinpath(PLOTS_DIR, "Poincaré map.png"));

println()

# Show the plot
p

end

# ╔═╡ Cell order:
# ╟─6a87a898-9dfd-11ec-2b38-49b883798d79
# ╟─4253f362-f966-45af-ac9c-0016672f5952
# ╟─4fe5044e-4f67-4bb5-afe9-f5f95ac02c78
