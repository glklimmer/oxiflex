import json
import matplotlib.pyplot as plt

# Load the JSON data from the specified file path
with open('Problems/queens/data/iterations_inference.json', 'r') as file:
    data = json.load(file)

# Prepare plot data
n_values = sorted(data.keys(), key=int)  # Sorting n values to ensure correct plot order
options = set(opt for values in data.values() for opt in values)  # Gather all unique options

# Create a dictionary to hold option results for each n
plot_data = {opt: [] for opt in options}
for n in n_values:
    for opt in options:
        # Extract the number before '±' and convert to integer for plotting
        result = data[n].get(opt, "0 ± 0").split("±")[0].strip()  # Using "0 ± 0" as default
        plot_data[opt].append(int(result))  # Append the integer result

# Mapping of flags to descriptive labels
flag_labels = {
    "-f": "FC w/ VO",
    "-a_1": "AC-1 w/ VO",
    "": "AC-3 w/ VO"
}

# Generate the plot
n_values_int = list(map(int, n_values))  # Convert n values from string to integers
for opt, results in plot_data.items():
    label = flag_labels.get(opt, "AC-3 w/ VO")  # Use the mapping to get descriptive labels
    plt.plot(n_values_int, results, label=label.replace("_", " ").strip())

# plt.title("N-Queens (Averaged, 5 Runs)")
plt.xlabel("n")
plt.ylabel("Iterations")
plt.xticks(n_values_int)  # Set x-axis ticks to the correct problem sizes
plt.legend()
plt.grid(True)
plt.savefig("Problems/queens/plots/iterations_inference.png")
