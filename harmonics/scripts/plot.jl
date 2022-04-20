# This script plots the objective
# function with different views

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
        julia --project=. scripts/plot.jl [--postfix <POSTFIX>] <INPUT> """
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

"Read binary files in the `bincode` format"
function read_bincode(path::AbstractString)::Tuple{I, Vector{F}}
    open(path, "r") do io
        n = read(io, I)
        a = reinterpret(F, read(io, sizeof(F) * n))
        n, a
    end
end

# Plot if the corresponding data files exist
if isfile(maximum_path) &&
    isfile(point_path) &&
    isfile(θ_path) &&
    isfile(φ_path) &&
    isfile(f_path)

    # Read the data
    _, maximum = read_bincode(maximum_path)
    _, point = read_bincode(point_path)
    n, θ = read_bincode(θ_path)
    _, φ = read_bincode(φ_path)
    _, f = read_bincode(f_path)

    # Normalize the vectors
    point ./= 2π
    θ ./= 2π
    φ ./= 2π

    println(" "^4, "> Plotting the heatmap...")

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
    scatter!([point[1]], [point[2]]; label = "maximum = $(maximum[1])", legend = :outertop, legendfontsize = 6)

    # Save the figure
    savefig(p, joinpath(PLOTS_DIR, "Heatmap$(POSTFIX).png"))

    println(" "^4, "> Plotting the surface plot...")

    # Plot the surface plot
    p = surface(
        θ,
        φ,
        reshape(f, (n, n));
        label = "",
        xlabel = L"\theta \; \mathrm{[2 \pi]}",
        ylabel = L"\varphi \; \mathrm{[2 \pi]}",
        zlabel = L"f",
    )

    # Point out the maximum
    scatter!([point[1]], [point[2]], maximum; label = "maximum = $(maximum[1])", legend = :outertop, legendfontsize = 6)

    # Save the figure
    savefig(p, joinpath(PLOTS_DIR, "Surface$(POSTFIX).png"))

    println(" "^4, "> Plotting the θ profile...")

    # Plot the surface plot
    p = surface(
        θ,
        φ,
        reshape(f, (n, n));
        camera = (0, 0),
        label = "",
        xlabel = L"\hspace{4.5} \theta \; \mathrm{[2 \pi]}",
        yaxis = false,
        yticks = false,
        zlabel = L"f",
        colorbar = false,
    )

    # Point out the maximum
    scatter!([point[1]], [point[2]], maximum; label = "maximum = $(maximum[1])", legend = :outertop, legendfontsize = 6)

    # Save the figure
    savefig(p, joinpath(PLOTS_DIR, "θ Profile$(POSTFIX).png"))

    println(" "^4, "> Plotting the φ profile...")

    # Plot the surface plot
    p = surface(
        θ,
        φ,
        reshape(f, (n, n));
        camera = (90, 0),
        label = "",
        xaxis = false,
        xticks = false,
        ylabel = L"\varphi \; \mathrm{[2 \pi]} \hspace{4.5}",
        zlabel = L"f",
        colorbar = false,
    )

    # Point out the maximum
    scatter!([point[1]], [point[2]], maximum; label = "maximum = $(maximum[1])", legend = :outertop, legendfontsize = 6)

    # Save the figure
    savefig(p, joinpath(PLOTS_DIR, "φ Profile$(POSTFIX).png"))
end

println()
