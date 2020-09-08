import matplotlib.pyplot as plt
import pandas as pd

# Load csv
df = pd.read_csv("bench.csv")

# Filtering
peroxide = df[df["command"].str.contains("peroxide")]
eigen3 = df[df["command"].str.contains("eigen3")]
numpy = df[df["command"].str.contains("numpy")]
julia = df[df["command"].str.contains("julia")]
chapel = df[df["command"].str.contains("chapel")]

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for matmul", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time', fontsize=14)


# Plot with Legends
plt.plot(peroxide["parameter"], peroxide["mean"], marker='o', label=r'peroxide')
plt.fill_between(peroxide["parameter"], peroxide["min"], peroxide["max"], alpha=0.2)

plt.plot(eigen3["parameter"], eigen3["mean"], marker='o', label=r'eigen3')
plt.fill_between(eigen3["parameter"], eigen3["min"], eigen3["max"], alpha=0.2)

plt.plot(numpy["parameter"], numpy["mean"], marker='o', label=r'numpy')
plt.fill_between(numpy["parameter"], numpy["min"], numpy["max"], alpha=0.2)

plt.plot(julia["parameter"], julia["mean"], marker='o', label=r'julia')
plt.fill_between(julia["parameter"], julia["min"], julia["max"], alpha=0.2)

plt.plot(chapel["parameter"], chapel["mean"], marker='o', label=r'chapel')
plt.fill_between(chapel["parameter"], chapel["min"], chapel["max"], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
