#!/usr/bin/env python

"""This program shows parametrized `hyperfine` benchmark results as an
errorbar plot, with lines grouped by command parameters."""

import argparse
import json
import matplotlib.pyplot as plt
import sys
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("file", help="JSON file with benchmark results", nargs="+")
parser.add_argument(
    "--log-x", help="Use a logarithmic x (parameter) axis", action="store_true"
)
parser.add_argument(
    "--log-time", help="Use a logarithmic time axis", action="store_true"
)
parser.add_argument(
    "--titles", help="Comma-separated list of titles for the plot legend"
)
parser.add_argument(
    "-o", "--output", help="Save image to the given filename."
)

args = parser.parse_args()

def die(msg):
    sys.stderr.write("fatal: %s\n" % (msg,))
    sys.exit(1)

def extract_parameters(results):
    """Return `(parameter_name: str, parameter_values: List[float])`."""
    if not results:
        die("no benchmark data to plot")
    (names, values) = zip(*(unique_parameter(b) for b in results))
    names = frozenset(names)
    if len(names) != 1:
        die(
            "benchmarks must all have the same parameter name, but found: %s"
            % sorted(names)
        )
    return (next(iter(names)), list(values))

def unique_parameter(benchmark):
    """Return the unique parameter `(name: str, value: float)`, or die."""
    params_dict = benchmark.get("parameters", {})
    if not params_dict:
        die("benchmarks must have exactly one parameter, but found none")
    if len(params_dict) > 1:
        die(
            "benchmarks must have exactly one parameter, but found multiple: %s"
            % sorted(params_dict)
        )
    [(name, value)] = params_dict.items()
    return (name, float(value))

def extract_flags(command):
    """Extract flags from the command or provide a default label for no flags."""
    flags = re.findall(r"-\w+\s*[\w\d]*", command)
    flag_string = " ".join(sorted(flags)).strip()
    return flag_string if flag_string else "No Flags"

groups = {}  # Dictionary to hold data grouped by flags
parameter_name = None

for filename in args.file:
    with open(filename) as f:
        file_results = json.load(f)["results"]

    for result in file_results:
        flags = extract_flags(result["command"])
        if flags not in groups:
            groups[flags] = {
                "parameter_values": [],
                "times_mean": [],
                "times_stddev": []
            }

        (this_parameter_name, parameter_value) = unique_parameter(result)
        if parameter_name is not None and this_parameter_name != parameter_name:
            die(
                "files must all have the same parameter name, but found %r vs. %r"
                % (parameter_name, this_parameter_name)
            )
        parameter_name = this_parameter_name

        groups[flags]["parameter_values"].append(parameter_value)
        groups[flags]["times_mean"].append(result["mean"])
        groups[flags]["times_stddev"].append(result["stddev"])

all_parameter_values = set()
for data in groups.values():
    all_parameter_values.update(data["parameter_values"])

# Convert to sorted list and set as x-axis ticks
all_parameter_values = sorted(all_parameter_values)
plt.xticks(all_parameter_values)

# Plotting
for flags, data in groups.items():
    plt.plot(
        data["parameter_values"],
        data["times_mean"],
        label=flags
    )

plt.xlabel(parameter_name)
plt.ylabel("Time [s]")

if args.log_time:
    plt.yscale("log")

if args.log_x:
    plt.xscale("log")

plt.legend()
if args.output:
    plt.savefig(args.output)
else:
    plt.show()
