import pytest
import numpy as np

def np_sum(p):
    ar = np.arange(0, 10**p, 1)
    return ar.sum()

def normal_sum(p):
    ar = [i for i in range(0, 10**p)]
    return sum(ar)

def test_np_sum_4(benchmark):
    benchmark(np_sum, 4)

def test_np_sum_5(benchmark):
    benchmark(np_sum, 5)

def test_np_sum_6(benchmark):
    benchmark(np_sum, 6)

def test_np_sum_7(benchmark):
    benchmark(np_sum, 7)

def test_np_sum_8(benchmark):
    benchmark(np_sum, 8)

def test_normal_sum_4(benchmark):
    benchmark(normal_sum, 4)

def test_normal_sum_5(benchmark):
    benchmark(normal_sum, 5)

def test_normal_sum_6(benchmark):
    benchmark(normal_sum, 6)

def test_normal_sum_7(benchmark):
    benchmark(normal_sum, 7)

def test_normal_sum_8(benchmark):
    benchmark(normal_sum, 8)

