import matplotlib.pyplot as plt
import pandas as pd
import json
import pprint

# Load csv
df = pd.read_csv("bench.csv")
dj1 = pd.read_csv("bench_julia_for.csv")
dj2 = pd.read_csv("bench_julia_simd.csv")
dj3 = pd.read_csv("bench_julia_reduce.csv")
dj4 = pd.read_csv("bench_julia_avx.csv")
dp = pd.read_csv("bench_python.csv")

# Load Criterion data
def cum_criterion(path, min_vec, mean_vec, max_vec):
    with open(path, "r") as rust_json:
        rust_dict = json.load(rust_json)
        rust_mean = rust_dict["mean"]
        rust_conf = rust_mean["confidence_interval"]
        min_vec.append(rust_conf["lower_bound"] / 1E+9)
        mean_vec.append(rust_mean["point_estimate"] / 1E+9)
        max_vec.append(rust_conf["upper_bound"] / 1E+9)

c1_min = []
c1_mean = []
c1_max = []
c2_min = []
c2_mean = []
c2_max = []
c3_min = []
c3_mean = []
c3_max = []

small_range = [p*10000 for p in range(1, 11)]
large_range = [p*10000000 for p in range(1, 11)]

for param in small_range:
    cum_criterion(f"rust_sum/target/criterion/sum/for_sum/{param}/new/estimates.json", c1_min, c1_mean, c1_max)
    cum_criterion(f"rust_sum/target/criterion/sum/simd_sum/{param}/new/estimates.json", c2_min, c2_mean, c2_max)
    cum_criterion(f"rust_sum/target/criterion/sum/chunk_sum/{param}/new/estimates.json", c3_min, c3_mean, c3_max)

for param in large_range:
    cum_criterion(f"rust_sum/target/criterion/sum/for_sum/{param}/new/estimates.json", c1_min, c1_mean, c1_max)
    cum_criterion(f"rust_sum/target/criterion/sum/simd_sum/{param}/new/estimates.json", c2_min, c2_mean, c2_max)
    cum_criterion(f"rust_sum/target/criterion/sum/chunk_sum/{param}/new/estimates.json", c3_min, c3_mean, c3_max)

dc1 = pd.DataFrame({
        "min" : c1_min,
        "mean": c1_mean,
        "max" : c1_max,
    })

print(dc1)

dc2 = pd.DataFrame({
        "min" : c2_min,
        "mean": c2_mean,
        "max" : c2_max,
    })

dc3 = pd.DataFrame({
        "min" : c3_min,
        "mean": c3_mean,
        "max" : c3_max,
    })

# Filtering
rust_for = dc1
rust_simd = dc2
rust_chunk = dc3
#rust_for = df[df["command"].str.contains("rust_for")]
#rust_simd = df[df["command"].str.contains("rust_simd")]
#rust_chunk = df[df["command"].str.contains("rust_chunk")]
#eigen3 = df[df["command"].str.contains("default")]
#eigen3_blas = df[df["command"].str.contains("blas")]
#numpy = df[df["command"].str.contains("numpy")]
julia_for = dj1
julia_simd = dj2
julia_reduce = dj3
julia_avx = dj4
py_numpy = dp[dp["name"].str.contains("np")]
#py_sum = dp[dp["name"].str.contains("normal")]

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for summation", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

domain = small_range

# Plot with Legends
plt.plot(domain, py_numpy["mean"][0:10], marker='o', label=r'python(numpy)')
plt.fill_between(domain, py_numpy["min"][0:10], py_numpy["max"][0:10], alpha=0.2)

#plt.plot(domain, py_sum["mean"][0:10], marker='o', label=r'python(sum)')
#plt.fill_between(domain, py_sum["min"][0:10], py_sum["max"][0:10], alpha=0.2)

plt.plot(domain, julia_for["mean"][0:10], marker='o', label=r'julia(for loop)')
plt.fill_between(domain, julia_for["min"][0:10], julia_for["max"][0:10], alpha=0.2)

plt.plot(domain, julia_simd["mean"][0:10], marker='o', label=r'julia(simd)')
plt.fill_between(domain, julia_simd["min"][0:10], julia_simd["max"][0:10], alpha=0.2)

#plt.plot(domain, julia_reduce["mean"][0:10], marker='o', label=r'julia(reduce)')
#plt.fill_between(domain, julia_reduce["min"][0:10], julia_reduce["max"][0:10], alpha=0.2)
#
plt.plot(domain, julia_avx["mean"][0:10], marker='o', label=r'julia(avx)')
plt.fill_between(domain, julia_avx["min"][0:10], julia_avx["max"][0:10], alpha=0.2)

plt.plot(domain, rust_for["mean"][0:10], marker='o', label=r'rust(for loop)')
plt.fill_between(domain, rust_for["min"][0:10], rust_for["max"][0:10], alpha=0.2)

plt.plot(domain, rust_simd["mean"][0:10], marker='o', label=r'rust(simd)')
plt.fill_between(domain, rust_simd["min"][0:10], rust_simd["max"][0:10], alpha=0.2)

plt.plot(domain, rust_chunk["mean"][0:10], marker='o', label=r'rust(chunk)')
plt.fill_between(domain, rust_chunk["min"][0:10], rust_chunk["max"][0:10], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("small_plot.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for summation", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

domain = large_range

# Plot with Legends
plt.plot(domain, rust_for["mean"][10:], marker='o', label=r'rust(for loop)')
plt.fill_between(domain, rust_for["min"][10:], rust_for["max"][10:], alpha=0.2)

plt.plot(domain, rust_simd["mean"][10:], marker='o', label=r'rust(simd)')
plt.fill_between(domain, rust_simd["min"][10:], rust_simd["max"][10:], alpha=0.2)

plt.plot(domain, rust_chunk["mean"][10:], marker='o', label=r'rust(chunk)')
plt.fill_between(domain, rust_chunk["min"][10:], rust_chunk["max"][10:], alpha=0.2)

plt.plot(domain, julia_for["mean"][10:], marker='o', label=r'julia(for loop)')
plt.fill_between(domain, julia_for["min"][10:], julia_for["max"][10:], alpha=0.2)

plt.plot(domain, julia_simd["mean"][10:], marker='o', label=r'julia(simd)')
plt.fill_between(domain, julia_simd["min"][10:], julia_simd["max"][10:], alpha=0.2)

#plt.plot(domain, julia_reduce["mean"][10:], marker='o', label=r'julia(reduce)')
#plt.fill_between(domain, julia_reduce["min"][10:], julia_reduce["max"][10:], alpha=0.2)
#
plt.plot(domain, julia_avx["mean"][10:], marker='o', label=r'julia(avx)')
plt.fill_between(domain, julia_avx["min"][10:], julia_avx["max"][10:], alpha=0.2)

plt.plot(domain, py_numpy["mean"][10:], marker='o', label=r'python(numpy)')
plt.fill_between(domain, py_numpy["min"][10:], py_numpy["max"][10:], alpha=0.2)

#plt.plot(domain, py_sum["mean"], marker='o', label=r'python(sum)')
#plt.fill_between(domain, py_sum["min"], py_sum["max"], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("large_plot.png", dpi=300)
