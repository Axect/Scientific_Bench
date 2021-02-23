import matplotlib.pyplot as plt
import pandas as pd

# Load csv
df = pd.read_csv("bench.csv")
dj1 = pd.read_csv("bench_julia_for.csv")
dj2 = pd.read_csv("bench_julia_simd.csv")
dj3 = pd.read_csv("bench_julia_reduce.csv")

# Filtering
rust_for = df[df["command"].str.contains("rust_for")]
rust_simd = df[df["command"].str.contains("rust_simd")]
#eigen3 = df[df["command"].str.contains("default")]
#eigen3_blas = df[df["command"].str.contains("blas")]
#numpy = df[df["command"].str.contains("numpy")]
julia_for = dj1
julia_simd = dj2
julia_reduce = dj3
#chapel = df[df["command"].str.contains("chapel")]
#o3 = df[df["command"].str.contains("o3")]
#nim = df[df["command"].str.contains("nim")]

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for summation", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

domain = [10**p for p in rust_for["parameter_p"]]

# Plot with Legends
plt.semilogx(domain, rust_for["mean"], marker='o', label=r'rust(for loop)')
plt.fill_between(domain, rust_for["min"], rust_for["max"], alpha=0.2)

plt.semilogx(domain, rust_simd["mean"], marker='o', label=r'rust(simd)')
plt.fill_between(domain, rust_simd["min"], rust_simd["max"], alpha=0.2)

#plt.plot(eigen3["parameter_p"], eigen3["mean"], marker='o', label=r'eigen3')
#plt.fill_between(eigen3["parameter_p"], eigen3["min"], eigen3["max"], alpha=0.2)
#
#plt.plot(nim["parameter_p"], nim["mean"], marker='o', label=r'arraymancer')
#plt.fill_between(nim["parameter_p"], nim["min"], nim["max"], alpha=0.2)

plt.semilogx(domain, julia_for["mean"], marker='o', label=r'julia(for loop)')
plt.fill_between(domain, julia_for["min"], julia_for["max"], alpha=0.2)

plt.semilogx(domain, julia_simd["mean"], marker='o', label=r'julia(simd)')
plt.fill_between(domain, julia_simd["min"], julia_simd["max"], alpha=0.2)

plt.semilogx(domain, julia_reduce["mean"], marker='o', label=r'julia(reduce)')
plt.fill_between(domain, julia_reduce["min"], julia_reduce["max"], alpha=0.2)

#plt.plot(numpy["parameter_p"], numpy["mean"], marker='o', label=r'numpy')
#plt.fill_between(numpy["parameter_p"], numpy["min"], numpy["max"], alpha=0.2)
#
#plt.plot(eigen3_blas["parameter_p"], eigen3_blas["mean"], marker='o', label=r'eigen3(blas)')
#plt.fill_between(eigen3_blas["parameter_p"], eigen3_blas["min"], eigen3_blas["max"], alpha=0.2)


#plt.plot(chapel["parameter_p"], chapel["mean"], marker='o', label=r'chapel')
#plt.fill_between(chapel["parameter_p"], chapel["min"], chapel["max"], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
