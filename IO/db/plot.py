import matplotlib.pyplot as plt
from matplotlib import ticker
import scienceplots
import numpy as np
import pandas as pd

df_write = pd.read_csv('./bench_write.csv')['mean']
df_read = pd.read_csv('./bench_read.csv')['mean']
df_update = pd.read_csv('./bench_update.csv')['mean']

# Prepare Data to Plot
dummy = np.array([0, 1, 2, 3])
dbs = ["NativeDB", "JammDB", "Files+netCDF", "Numpy"]
data = {
    'Read': np.array(df_read),
    'Update': np.array(df_update),
    'Write': np.array(df_write),
    'Size': np.array([787.5, 888.1, 1.5*1000, 1.1*1000]) / 1000
}

width = 0.2
multiplier = 0

# Plot params
pparam = dict(
    ylabel = r'Time (s)',
    title = r"Benchmark for Database",
    xscale = 'linear',
    yscale = 'log',
    ylim = (1/1000, 40)
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots(figsize=(8, 4))
    ax.autoscale(tight=True)
    ax.set(**pparam)
    for i, (name, values) in enumerate(data.items()):
        offset = width * multiplier
        rects = ax.bar(dummy + offset - 0.1, values, width, label=name)
        if i == 3:
            ax.bar_label(rects, fmt='%.2fGB', padding=2, fontsize=5.5)
        else:
            ax.bar_label(rects, fmt='%.3f', padding=2, fontsize=6)
        multiplier += 1
    ax.legend(fontsize=8, ncol=4)
    ax.set_xticks(dummy + width, dbs)
    # Disable minor ticks
    ax.get_xaxis().set_minor_locator(ticker.NullLocator())
    ax.get_yaxis().set_minor_locator(ticker.NullLocator())
    fig.savefig('bench_plot.png', dpi=300, bbox_inches='tight')
