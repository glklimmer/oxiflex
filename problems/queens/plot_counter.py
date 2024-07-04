import json
import matplotlib.pyplot as plt

# Load the JSON data from the specified file path
with open('problems/queens/count_logging.json', 'r') as file:
    data = json.load(file)

# Prepare plot data
n_values = sorted(data.keys(), key=int)  # Sorting n values to ensure correct plot order
options = set(opt for values in data.values() for opt in values)  # Gather all unique options

# Create a dictionary to hold option results for each n
plot_data = {opt: [] for opt in options}
for n in n_values:
    for opt in options:
        # Append the result to the appropriate option, converting to integer for plotting
        plot_data[opt].append(int(data[n].get(opt, 0)))  # Using 0 as default if option is missing

# Generate the plot
n_values_int = list(map(int, n_values))  # Convert n values from string to integers
plt.figure(figsize=(12, 8))
for opt, results in plot_data.items():
    label = opt if opt else "no flags"  # Adjusting label for "no flags" option
    plt.plot(n_values_int, results, label=label.replace("_", " ").strip())

plt.title("Benchmark Results by Problem Size and Option (Averaged over 5 Runs)")
plt.xlabel("n")
plt.ylabel("Average Result")
plt.xticks(n_values_int)  # Set x-axis ticks to the correct problem sizes
plt.legend(title="Options", loc='upper left')
plt.grid(True)
plt.savefig("problems/queens/count_logging.png")
