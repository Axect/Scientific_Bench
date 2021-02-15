import matplotlib.pyplot as plt
import pandas as pd

# Load csv
df = pd.read_csv("bench.csv")

# Filtering
peroxide = df[df["command"].str.contains("peroxide")]
eigen3 = df[df["command"].str.contains("eigen3")]
numpy = df[df["command"].str.contains("numpy")]
julia = df[df["command"].str.contains("julia")]
#chapel = df[df["command"].str.contains("chapel")]
o3 = df[df["command"].str.contains("o3")]

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for matmul", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)


# Plot with Legends
plt.plot(peroxide["parameter_size"], peroxide["mean"], marker='o', label=r'peroxide')
plt.fill_between(peroxide["parameter_size"], peroxide["min"], peroxide["max"], alpha=0.2)

plt.plot(o3["parameter_size"], o3["mean"], marker='o', label=r'peroxide(O3)')
plt.fill_between(o3["parameter_size"], o3["min"], o3["max"], alpha=0.2)

plt.plot(eigen3["parameter_size"], eigen3["mean"], marker='o', label=r'eigen3')
plt.fill_between(eigen3["parameter_size"], eigen3["min"], eigen3["max"], alpha=0.2)

plt.plot(numpy["parameter_size"], numpy["mean"], marker='o', label=r'numpy')
plt.fill_between(numpy["parameter_size"], numpy["min"], numpy["max"], alpha=0.2)

plt.plot(julia["parameter_size"], julia["mean"], marker='o', label=r'julia')
plt.fill_between(julia["parameter_size"], julia["min"], julia["max"], alpha=0.2)

#plt.plot(chapel["parameter_size"], chapel["mean"], marker='o', label=r'chapel')
#plt.fill_between(chapel["parameter_size"], chapel["min"], chapel["max"], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
