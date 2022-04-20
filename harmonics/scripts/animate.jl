# This script animates the optimization process

# Define the floating point type used across the script
F = Float64

# Define the integer type used across the script
I = UInt64

# Define default values for optional arguments
POSTFIX = ""

"Check if the value of the option is the last argument"
function check_last(i)
    if i + 1 == length(ARGS)
        println("The last argument should be the input directory.")
        exit(1)
    end
end

# Parse the options
for i in eachindex(ARGS)
    # A postfix for the names of output files
    if ARGS[i] == "--postfix"
        check_last(i)
        try
            global POSTFIX = " ($(ARGS[i+1]))"
        catch
            println("Couldn't parse the value of the `--postfix` argument.")
            exit(1)
        end
    end
end

# Check for required arguments
if length(ARGS) == 0
    println("""
        Usage:
        julia --project=. scripts/animate.jl [--postfix <POSTFIX>] <INPUT> """
    )
    exit(1)
end

# Define the input directory
INPUT_DIR = ARGS[end]

println('\n', " "^4, "> Loading the packages...")

using LaTeXStrings
using Plots

# Use the GR backend for plots
gr()

# Change some of the default parameters for plots
default(fontfamily = "Computer Modern", dpi = 300, legend = :topright, size = (400, 400))

# Define the paths to output directories
CURRENT_DIR = @__DIR__
ROOT_DIR = basename(CURRENT_DIR) == "scripts" ? dirname(CURRENT_DIR) : CURRENT_DIR
PLOTS_DIR = joinpath(ROOT_DIR, "plots")

# Make sure the needed directories exist
mkpath(PLOTS_DIR)

# Define the paths to the binary files
maximum_path = joinpath(INPUT_DIR, "maximum.bin")
point_path = joinpath(INPUT_DIR, "point.bin")
θ_path = joinpath(INPUT_DIR, "theta.bin")
φ_path = joinpath(INPUT_DIR, "phi.bin")
f_path = joinpath(INPUT_DIR, "obj.bin")
ts_path = joinpath(INPUT_DIR, "ts.bin")
ps_path = joinpath(INPUT_DIR, "ps.bin")
fs_path = joinpath(INPUT_DIR, "fs.bin")
best_ps_path = joinpath(INPUT_DIR, "best_ps.bin")
best_fs_path = joinpath(INPUT_DIR, "best_fs.bin")

"Read binary files in the `bincode` format"
function read_bincode(path::AbstractString)::Tuple{I, Vector{F}}
    open(path, "r") do io
        n = read(io, I)
        a = reinterpret(F, read(io, sizeof(F) * n))
        n, a
    end
end

# Animate if the corresponding data files exist
if isfile(maximum_path) &&
    isfile(point_path) &&
    isfile(θ_path) &&
    isfile(φ_path) &&
    isfile(f_path) &&
    isfile(ts_path) &&
    isfile(ps_path) &&
    isfile(fs_path) &&
    isfile(best_ps_path) &&
    isfile(best_fs_path)

    # Read the data
    _, maximum_f = read_bincode(maximum_path)
    _, point = read_bincode(point_path)
    n, θ = read_bincode(θ_path)
    _, φ = read_bincode(φ_path)
    _, f = read_bincode(f_path)
    m, ts = read_bincode(ts_path)
    _, ps = read_bincode(ps_path)
    _, fs = read_bincode(fs_path)
    _, best_ps = read_bincode(best_ps_path)
    _, best_fs = read_bincode(best_fs_path)

    # Normalize the vectors
    point ./= 2π
    θ ./= 2π
    φ ./= 2π
    ps ./= 2π
    best_ps ./= 2π

    println(" "^4, "> Animating... ($(m) frames)")

    # Plot the heatmap
    p = heatmap(
        θ,
        φ,
        reshape(f, (n, n));
        label = "",
        xlabel = L"\theta \; \mathrm{[2 \pi]}",
        ylabel = L"\varphi \; \mathrm{[2 \pi]}",
        zlabel = L"f",
    )

    # Point out the maximum
    scatter!([point[1]], [point[2]]; label = "maximum = $(maximum_f[1])", legend = :outertop, legendfontsize = 6)

    # Create an animation
    anim = @animate for i in 1:m
        ix = 1 + 2 * (i - 1)
        scatter(p, [ps[ix]], [ps[ix+1]]; label = rpad("current point = $(round(fs[i]; digits=14))", 32, "0"))
        scatter!([best_ps[ix]], [best_ps[ix+1]]; label = "best point = $(best_fs[i])")
        scatter!([0.5], [0.5], label = rpad("temperature = $(round(ts[i]; digits=14))", 31, "0"), markeralpha = 0)
    end

    # Save the figure
    gif(anim, joinpath(PLOTS_DIR, "Optimization.mp4"), fps = 15, show_msg = false)
end

println()
