import numpy as np

ROW = 100
COL = 100

m = np.arange(0,ROW*COL,1).reshape(ROW,COL)
n = m

result = np.matmul(m,n)

print(result[ROW//2, COL//2])
