# This is a Julia script for plotting the
# orbits that show oscillations (position vs. time)

# Define the floating point type used across the script
F = Float64

# Define the integer type used across the script
I = UInt64

# Define default values for optional arguments
POSTFIX = ""
H = 0.01

"Check if the value of the option is the last argument"
function check_last(i)
    if i + 1 == length(ARGS)
        println("The last argument should be the input directory.")
        exit(1)
    end
end

# Parse the options
for i in eachindex(ARGS)
    # Time step
    if ARGS[i] == "-h"
        check_last(i)
        try
            global H = parse(F, ARGS[i+1])
        catch
            println("Couldn't parse the value of the `-h` argument.")
            exit(1)
        end
    end
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
        julia --project=. scripts/oscillations.jl [--postfix <POSTFIX>] <INPUT> """
    )
    exit(1)
end

# Multiply the time step
H *= π / 2

# Define the input directory
INPUT_DIR = ARGS[end]

println('\n', " "^4, "> Loading the packages...")

using LaTeXStrings
using Plots

# Use the GR backend for plots
gr()

# Change some of the default parameters for plots
default(fontfamily = "Computer Modern", dpi = 300, legend = :topright)

# Define the paths to output directories
CURRENT_DIR = @__DIR__
ROOT_DIR = basename(CURRENT_DIR) == "scripts" ? dirname(CURRENT_DIR) : CURRENT_DIR
PLOTS_DIR = joinpath(ROOT_DIR, "plots")
OUTPUT_DIR = joinpath(PLOTS_DIR, "oscillations")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

# Define the paths to the binary files
z_path = joinpath(INPUT_DIR, "z.bin")

"Read binary files in the `bincode` format"
function read_bincode(path::AbstractString)::Tuple{I, Vector{F}}
    open(path, "r") do io
        n = read(io, I)
        a = reinterpret(F, read(io, sizeof(F) * n))
        n, a
    end
end

# Plot the phase space portrait if the corresponding data files exist
if isfile(z_path)
    # Read the data
    n, z = read_bincode(z_path)

    println(" "^4, "> Plotting the orbit...")

    # Plot the phase space portrait
    p = plot(
        [ i * H / (2 * π) for i in 0:(n-1) ],
        z;
        label = "",
        title = "Orbit",
        xlabel = L"t \; \mathrm{[2 \pi]}",
        ylabel = L"z",
        xminorticks = 5,
    )

    # Save the figure as PDF and PNG
    savefig(p, joinpath(OUTPUT_DIR, "Orbit$(POSTFIX).pdf"))
    savefig(p, joinpath(OUTPUT_DIR, "Orbit$(POSTFIX).png"))
end

println()
