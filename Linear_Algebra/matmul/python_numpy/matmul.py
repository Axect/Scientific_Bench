import numpy as np
import sys

args = sys.argv[1:]
ROW = int(args[0])
COL = int(args[1])

m = np.random.rand(ROW, COL)
n = np.random.rand(ROW, COL)

result = np.matmul(m,n)

print(result[ROW//2, COL//2])
