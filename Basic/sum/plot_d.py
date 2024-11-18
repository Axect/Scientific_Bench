import matplotlib.pyplot as plt
import pandas as pd
import json
import pprint

# Load D data
def read_d(path, name, min_vec, mean_vec, max_vec):
    with open(path, "r") as d_json:
        d_dict = json.load(d_json)
        for calculation in d_dict:
            for func_type in calculation["BenchmarksResults"]:
                if func_type["name"] == name:
                    min_vec.append(func_type["ns_iter_summ"]["min"] / 1E+9)
                    mean_vec.append(func_type["ns_iter_summ"]["mean"] / 1E+9)
                    max_vec.append(func_type["ns_iter_summ"]["max"] / 1E+9)

d1_min = []
d1_mean = []
d1_max = []
d2_min = []
d2_mean = []
d2_max = []
d3_min = []
d3_mean = []
d3_max = []
d4_min = []
d4_mean = []
d4_max = []

small_range = [p*10000 for p in range(1, 11)]
large_range = [p*10000000 for p in range(1, 11)]

read_d("sum/d_sum/data_small.json", "test_sum__loop", d1_min, d1_mean, d1_max)
read_d("sum/d_sum/data_small.json", "test_sum__func", d2_min, d2_mean, d2_max)
read_d("sum/d_sum/data_small.json", "test_sum__simd", d3_min, d3_mean, d3_max)
read_d("sum/d_sum/data_small.json", "test_sum_autov", d4_min, d4_mean, d4_max)

read_d("sum/d_sum/data_large.json", "test_sum__loop", d1_min, d1_mean, d1_max)
read_d("sum/d_sum/data_large.json", "test_sum__func", d2_min, d2_mean, d2_max)
read_d("sum/d_sum/data_large.json", "test_sum__simd", d3_min, d3_mean, d3_max)
read_d("sum/d_sum/data_large.json", "test_sum_autov", d4_min, d4_mean, d4_max)

d1 = pd.DataFrame({
        "min" : d1_min,
        "mean": d1_mean,
        "max" : d1_max,
    })

d2 = pd.DataFrame({
        "min" : d2_min,
        "mean": d2_mean,
        "max" : d2_max,
    })

d3 = pd.DataFrame({
        "min" : d3_min,
        "mean": d3_mean,
        "max" : d3_max,
    })

d4 = pd.DataFrame({
        "min" : d4_min,
        "mean": d4_mean,
        "max" : d4_max,
    })

# Filtering
d_loop = d1
d_func = d2
d_autovec = d3
d_simd = d4

# Use latex
#plt.rc('text', usetex=True)
#plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for summation (small)", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

domain = small_range

# Plot with Legends
plt.plot(domain, d_loop["mean"][0:10], marker='o', label=r'd(for loop)')
plt.fill_between(domain, d_loop["min"][0:10], d_loop["max"][0:10], alpha=0.2)

plt.plot(domain, d_func["mean"][0:10], marker='o', label=r'd(function)')
plt.fill_between(domain, d_func["min"][0:10], d_func["max"][0:10], alpha=0.2)

plt.plot(domain, d_simd["mean"][0:10], marker='o', label=r'd(simd)')
plt.fill_between(domain, d_simd["min"][0:10], d_simd["max"][0:10], alpha=0.2)

plt.plot(domain, d_autovec["mean"][0:10], marker='o', label=r'd(auto-vec)')
plt.fill_between(domain, d_autovec["min"][0:10], d_autovec["max"][0:10], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("small_plot.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for summation (large)", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

domain = large_range

# Plot with Legends
plt.plot(domain, d_loop["mean"][10:20], marker='o', label=r'd(for loop)')
plt.fill_between(domain, d_loop["min"][10:20], d_loop["max"][10:20], alpha=0.2)

plt.plot(domain, d_func["mean"][10:20], marker='o', label=r'd(function)')
plt.fill_between(domain, d_func["min"][10:20], d_func["max"][10:20], alpha=0.2)

plt.plot(domain, d_simd["mean"][10:20], marker='o', label=r'd(simd)')
plt.fill_between(domain, d_simd["min"][10:20], d_simd["max"][10:20], alpha=0.2)

plt.plot(domain, d_autovec["mean"][10:20], marker='o', label=r'd(auto-vec)')
plt.fill_between(domain, d_autovec["min"][10:20], d_autovec["max"][10:20], alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("large_plot.png", dpi=300)
