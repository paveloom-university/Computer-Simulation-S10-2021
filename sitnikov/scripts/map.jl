# This is a Julia script for plotting the Poincaré map

# Define the floating point type used across the script
F = Float64

# Define the integer type used across the script
I = UInt64

# Define default values for optional arguments
POSTFIX = ""
T = 1000

"Check if the value of the option is the last argument"
function check_last(i)
    if i + 1 == length(ARGS)
        println("The last argument should be the input directory.")
        exit(1)
    end
end

# Parse the options
for i in eachindex(ARGS)
    # Number of periods
    if ARGS[i] == "-t"
        check_last(i)
        try
            global T = parse(F, ARGS[i+1])
        catch
            println("Couldn't parse the value of the `-t` argument.")
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
        julia --project=. scripts/map.jl [-t <T>] [--postfix <POSTFIX>] <INPUT> """
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
default(fontfamily = "Computer Modern", dpi = 300, legend = :topright)

# Define the paths to output directories
CURRENT_DIR = @__DIR__
ROOT_DIR = basename(CURRENT_DIR) == "scripts" ? dirname(CURRENT_DIR) : CURRENT_DIR
PLOTS_DIR = joinpath(ROOT_DIR, "plots")
OUTPUT_DIR = joinpath(PLOTS_DIR, "portrait")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

# Define the paths to the binary files
z_path = joinpath(INPUT_DIR, "z.bin")
z_v_path = joinpath(INPUT_DIR, "z_v.bin")

"Read binary files in the `bincode` format"
function read_bincode(path::AbstractString)::Tuple{I, Vector{F}}
    open(path, "r") do io
        n = read(io, I)
        a = reinterpret(F, read(io, sizeof(F) * n))
        n, a
    end
end

# Plot the Poincaré map if the corresponding data files exist
if isfile(z_path) && isfile(z_v_path)
    # Read the data
    nz, z = read_bincode(z_path)
    nzv, z_v = read_bincode(z_v_path)

    # Compute the number of points per period
    np = UInt((nz - 1) / T)

    # Check if the lengths match
    if nz != nzv
        println(" "^4, "> [!] Lengths of the files for positions and velocities don't match")
        return
    end

    println(" "^4, "> Plotting the Poincaré map...")

    # Plot the Poincaré map
    p = scatter(
        z[1:np:end],
        z_v[1:np:end];
        label = "",
        title = "Poincaré map",
        xlabel = L"z",
        ylabel = L"\dot{z}",
        size = (400, 400),
        markersize = 0.5,
    )

    # Save the figure as PDF and PNG
    savefig(p, joinpath(OUTPUT_DIR, "Poincaré map$(POSTFIX).pdf"))
    savefig(p, joinpath(OUTPUT_DIR, "Poincaré map$(POSTFIX).png"))
end

println()
