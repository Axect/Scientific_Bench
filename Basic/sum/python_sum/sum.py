import pytest
import numpy as np

def np_sum(ar):
    return ar.sum()

def normal_sum(ar):
    return sum(ar)

np_range = [np.arange(0, p*10000, 1) for p in range(1, 11)]
large_np_range = [np.arange(0, p*10000000, 1) for p in range(1, 11)]
#small_range = [[p for p in range(0, 10000*n)] for n in range(1, 11)]
#large_range = [[p for p in range(0, 10000000*n)] for n in range(1, 11)]

np_range = np.concatenate((np_range, large_np_range))

def test_np_sum_10000(benchmark):
    benchmark(np_sum, np_range[0])

def test_np_sum_20000(benchmark):
    benchmark(np_sum, np_range[1])

def test_np_sum_30000(benchmark):
    benchmark(np_sum, np_range[2])

def test_np_sum_40000(benchmark):
    benchmark(np_sum, np_range[3])

def test_np_sum_50000(benchmark):
    benchmark(np_sum, np_range[4])

def test_np_sum_60000(benchmark):
    benchmark(np_sum, np_range[5])

def test_np_sum_70000(benchmark):
    benchmark(np_sum, np_range[6])

def test_np_sum_80000(benchmark):
    benchmark(np_sum, np_range[7])

def test_np_sum_90000(benchmark):
    benchmark(np_sum, np_range[8])

def test_np_sum_100000(benchmark):
    benchmark(np_sum, np_range[9])

def test_np_sum_10000000(benchmark):
    benchmark(np_sum, np_range[10])

def test_np_sum_20000000(benchmark):
    benchmark(np_sum, np_range[11])

def test_np_sum_30000000(benchmark):
    benchmark(np_sum, np_range[12])

def test_np_sum_40000000(benchmark):
    benchmark(np_sum, np_range[13])

def test_np_sum_50000000(benchmark):
    benchmark(np_sum, np_range[14])

def test_np_sum_60000000(benchmark):
    benchmark(np_sum, np_range[15])

def test_np_sum_70000000(benchmark):
    benchmark(np_sum, np_range[16])

def test_np_sum_80000000(benchmark):
    benchmark(np_sum, np_range[17])

def test_np_sum_90000000(benchmark):
    benchmark(np_sum, np_range[18])

def test_np_sum_100000000(benchmark):
    benchmark(np_sum, np_range[19])
