import pytest
import numpy as np
#import sys

#args = sys.argv[1:]
#ROW = int(args[0])
#COL = int(args[1])
#
#m = np.random.rand(ROW, COL)
#n = np.random.rand(ROW, COL)
#
#result = np.matmul(m,n)
#
#print(result[ROW//2, COL//2])

def mm(row):
    m = np.random.rand(row, row)
    n = np.random.rand(row, row)
    result = np.matmul(m,n)
    return result[row//2, row//2]

def test_mm_100(benchmark):
    benchmark(mm, 100)

def test_mm_200(benchmark):
    benchmark(mm, 200)

def test_mm_300(benchmark):
    benchmark(mm, 300)

def test_mm_400(benchmark):
    benchmark(mm, 400)

def test_mm_500(benchmark):
    benchmark(mm, 500)

def test_mm_600(benchmark):
    benchmark(mm, 600)

def test_mm_700(benchmark):
    benchmark(mm, 700)

def test_mm_800(benchmark):
    benchmark(mm, 800)

def test_mm_900(benchmark):
    benchmark(mm, 900)

def test_mm_1000(benchmark):
    benchmark(mm, 1000)
