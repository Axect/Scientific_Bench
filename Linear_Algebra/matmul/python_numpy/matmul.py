import numpy as np

ROW = 100
COL = 100

m = np.random.rand(ROW, COL)
n = np.random.rand(ROW, COL)

result = np.matmul(m,n)

print(result[ROW//2, COL//2])
