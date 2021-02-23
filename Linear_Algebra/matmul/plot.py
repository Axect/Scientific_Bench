import matplotlib.pyplot as plt
import pandas as pd

# Load csv
df = pd.read_csv("bench.csv")
dg = pd.read_csv("bench_julia.csv")
dp = pd.read_csv("bench_python.csv")

# Filtering
peroxide = df[df["command"].str.contains("peroxide")]
eigen3 = df[df["command"].str.contains("default")]
eigen3_blas = df[df["command"].str.contains("blas")]
numpy = dp
#julia = df[df["command"].str.contains("julia")]
julia = dg
#chapel = df[df["command"].str.contains("chapel")]
o3 = df[df["command"].str.contains("o3")]
nim = df[df["command"].str.contains("nim")]

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for matmul", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

domain = peroxide["parameter_size"]


# Plot with Legends
plt.plot(peroxide["parameter_size"], peroxide["mean"], marker='o', label=r'peroxide')
plt.fill_between(peroxide["parameter_size"], peroxide["min"], peroxide["max"], alpha=0.2)

plt.plot(o3["parameter_size"], o3["mean"], marker='o', label=r'peroxide(O3)')
plt.fill_between(o3["parameter_size"], o3["min"], o3["max"], alpha=0.2)

plt.plot(eigen3["parameter_size"], eigen3["mean"], marker='o', label=r'eigen3')
plt.fill_between(eigen3["parameter_size"], eigen3["min"], eigen3["max"], alpha=0.2)

plt.plot(nim["parameter_size"], nim["mean"], marker='o', label=r'arraymancer')
plt.fill_between(nim["parameter_size"], nim["min"], nim["max"], alpha=0.2)

plt.plot(julia["param"], julia["mean"], marker='o', label=r'julia')
plt.fill_between(julia["param"], julia["min"], julia["max"], alpha=0.2)

plt.plot(domain, numpy["mean"], marker='o', label=r'numpy')
plt.fill_between(domain, numpy["min"], numpy["max"], alpha=0.2)

plt.plot(eigen3_blas["parameter_size"], eigen3_blas["mean"], marker='o', label=r'eigen3(blas)')
plt.fill_between(eigen3_blas["parameter_size"], eigen3_blas["min"], eigen3_blas["max"], alpha=0.2)


#plt.plot(chapel["parameter_size"], chapel["mean"], marker='o', label=r'chapel')
#plt.fill_between(chapel["parameter_size"], chapel["min"], chapel["max"], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
