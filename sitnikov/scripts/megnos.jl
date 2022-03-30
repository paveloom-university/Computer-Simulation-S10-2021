# This is a Julia script for plotting
# the time evolution of MEGNOs

# Define the floating point type used across the script
F = Float64

# Define the integer type used across the script
I = UInt64

# Define default values for optional arguments
POSTFIX = ""
H = 0.01

# Define fixed parameters
I_M = round(UInt, 1 / H)

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
        julia --project=. scripts/megnos.jl [-h <H>] [--postfix <POSTFIX>] <INPUT> """
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
OUTPUT_DIR = joinpath(PLOTS_DIR, "megnos")

# Make sure the needed directories exist
mkpath(OUTPUT_DIR)

# Define the path to the binary file
megno_path = joinpath(INPUT_DIR, "megno.bin")
mean_megno_path = joinpath(INPUT_DIR, "mean_megno.bin")

"Read binary files in the `bincode` format"
function read_bincode(path::AbstractString)::Tuple{I, Vector{F}}
    open(path, "r") do io
        n = read(io, I)
        a = reinterpret(F, read(io, sizeof(F) * n))
        n, a
    end
end

# Plot the Poincaré map if the corresponding data files exist
if isfile(megno_path) && isfile(mean_megno_path)
    # Read the data
    n_megno, megno = read_bincode(megno_path)
    n_mean_megno, mean_megno = read_bincode(mean_megno_path)

    # Check if the lengths match
    if n_megno != n_mean_megno
        println(" "^4, "> [!] Lengths of the files for MEGNOs and mean MEGNOs don't match")
        return
    end

    # Compute the time scale
    n = n_megno
    time = [ (i + I_M) * H / (2 * π) for i in 0:(n-1) ]

    println(" "^4, "> Plotting the time evolution of MEGNOs...")

    # Plot the time evolution of MEGNOs
    p = plot(
        time,
        megno;
        label = "",
        title = "Time evolution of MEGNOs",
        xlabel = L"t \; \textrm{[2 \pi]}",
        ylabel = L"Y",
        titlefontsize = 12,
        size = (400, 400),
    )

    # Save the figure as PDF and PNG
    savefig(p, joinpath(OUTPUT_DIR, "MEGNOs$(POSTFIX).pdf"))
    savefig(p, joinpath(OUTPUT_DIR, "MEGNOs$(POSTFIX).png"))

    println(" "^4, "> Plotting the time evolution of mean MEGNOs...")

    # Plot the time evolution of mean MEGNOs
    p = plot(
        time,
        mean_megno;
        label = "",
        title = "Time evolution of mean MEGNOs",
        xlabel = L"t \; \textrm{[2 \pi]}",
        ylabel = L"\overline{Y}",
        titlefontsize = 12,
        size = (400, 400),
    )

    # Save the figure as PDF and PNG
    savefig(p, joinpath(OUTPUT_DIR, "Mean MEGNOs$(POSTFIX).pdf"))
    savefig(p, joinpath(OUTPUT_DIR, "Mean MEGNOs$(POSTFIX).png"))
end

println()
