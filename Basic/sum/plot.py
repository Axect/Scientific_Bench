import matplotlib.pyplot as plt
import pandas as pd

# Load csv
df = pd.read_csv("bench.csv")
dj1 = pd.read_csv("bench_julia_for.csv")
dj2 = pd.read_csv("bench_julia_simd.csv")
dj3 = pd.read_csv("bench_julia_reduce.csv")
dj4 = pd.read_csv("bench_julia_avx.csv")
dp = pd.read_csv("bench_python.csv")

# Filtering
rust_for = df[df["command"].str.contains("rust_for")]
rust_simd = df[df["command"].str.contains("rust_simd")]
rust_chunk = df[df["command"].str.contains("rust_chunk")]
#eigen3 = df[df["command"].str.contains("default")]
#eigen3_blas = df[df["command"].str.contains("blas")]
#numpy = df[df["command"].str.contains("numpy")]
julia_for = dj1
julia_simd = dj2
julia_reduce = dj3
julia_avx = dj4
py_numpy = dp[dp["name"].str.contains("np")]
py_sum = dp[dp["name"].str.contains("normal")]

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
plt.loglog(domain, py_numpy["mean"], marker='o', label=r'python(numpy)')
plt.fill_between(domain, py_numpy["min"], py_numpy["max"], alpha=0.2)

plt.loglog(domain, py_sum["mean"], marker='o', label=r'python(sum)')
plt.fill_between(domain, py_sum["min"], py_sum["max"], alpha=0.2)

plt.loglog(domain, julia_for["mean"], marker='o', label=r'julia(for loop)')
plt.fill_between(domain, julia_for["min"], julia_for["max"], alpha=0.2)

plt.loglog(domain, julia_simd["mean"], marker='o', label=r'julia(simd)')
plt.fill_between(domain, julia_simd["min"], julia_simd["max"], alpha=0.2)

plt.loglog(domain, julia_reduce["mean"], marker='o', label=r'julia(reduce)')
plt.fill_between(domain, julia_reduce["min"], julia_reduce["max"], alpha=0.2)

plt.loglog(domain, julia_avx["mean"], marker='o', label=r'julia(avx)')
plt.fill_between(domain, julia_avx["min"], julia_avx["max"], alpha=0.2)

plt.loglog(domain, rust_for["mean"], marker='o', label=r'rust(for loop)')
plt.fill_between(domain, rust_for["min"], rust_for["max"], alpha=0.2)

plt.loglog(domain, rust_simd["mean"], marker='o', label=r'rust(simd)')
plt.fill_between(domain, rust_simd["min"], rust_simd["max"], alpha=0.2)

plt.loglog(domain, rust_chunk["mean"], marker='o', label=r'rust(chunk)')
plt.fill_between(domain, rust_chunk["min"], rust_chunk["max"], alpha=0.2)



# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("log_log_plot.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for summation", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

# Plot with Legends
plt.semilogx(domain, rust_for["mean"], marker='o', label=r'rust(for loop)')
plt.fill_between(domain, rust_for["min"], rust_for["max"], alpha=0.2)

plt.semilogx(domain, rust_simd["mean"], marker='o', label=r'rust(simd)')
plt.fill_between(domain, rust_simd["min"], rust_simd["max"], alpha=0.2)

plt.semilogx(domain, rust_chunk["mean"], marker='o', label=r'rust(chunk)')
plt.fill_between(domain, rust_chunk["min"], rust_chunk["max"], alpha=0.2)

plt.semilogx(domain, julia_for["mean"], marker='o', label=r'julia(for loop)')
plt.fill_between(domain, julia_for["min"], julia_for["max"], alpha=0.2)

plt.semilogx(domain, julia_simd["mean"], marker='o', label=r'julia(simd)')
plt.fill_between(domain, julia_simd["min"], julia_simd["max"], alpha=0.2)

plt.semilogx(domain, julia_reduce["mean"], marker='o', label=r'julia(reduce)')
plt.fill_between(domain, julia_reduce["min"], julia_reduce["max"], alpha=0.2)

plt.semilogx(domain, julia_avx["mean"], marker='o', label=r'julia(avx)')
plt.fill_between(domain, julia_avx["min"], julia_avx["max"], alpha=0.2)

plt.semilogx(domain, py_numpy["mean"], marker='o', label=r'python(numpy)')
plt.fill_between(domain, py_numpy["min"], py_numpy["max"], alpha=0.2)

#plt.semilogx(domain, py_sum["mean"], marker='o', label=r'python(sum)')
#plt.fill_between(domain, py_sum["min"], py_sum["max"], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("semi_log_plot.png", dpi=300)
