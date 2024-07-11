import numpy as np
import sys

# Receive data from command line arguments
data = list(map(float, sys.argv[1:]))
sem = np.std(data, ddof=1) / np.sqrt(len(data))
print(f"{sem:.2f}")
