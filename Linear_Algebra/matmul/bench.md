| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./rust_peroxide/target/release/rust_peroxide 100 100` | 0.8 ± 0.1 | 0.6 | 1.4 | 1.00 |
| `./rust_o3/target/release/rust_o3 100 100` | 5.5 ± 2.9 | 1.9 | 22.7 | 7.39 ± 4.00 |
| `./cpp_eigen3/bin/matmul 100 100` | 1.1 ± 0.1 | 0.9 | 2.1 | 1.50 ± 0.26 |
| `./chapel/bin/matmul --r=100 --c=100` | 59.1 ± 12.8 | 40.5 | 86.6 | 78.69 ± 20.11 |
| `python python_numpy/matmul.py 100 100` | 90.6 ± 9.9 | 81.4 | 112.0 | 120.68 ± 21.01 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 100 100` | 183.2 ± 13.3 | 176.0 | 224.8 | 243.94 ± 37.47 |
| `./rust_peroxide/target/release/rust_peroxide 200 200` | 2.1 ± 0.2 | 1.8 | 3.3 | 2.79 ± 0.44 |
| `./rust_o3/target/release/rust_o3 200 200` | 7.1 ± 2.3 | 3.6 | 11.0 | 9.51 ± 3.31 |
| `./cpp_eigen3/bin/matmul 200 200` | 2.9 ± 0.2 | 2.5 | 3.5 | 3.88 ± 0.60 |
| `./chapel/bin/matmul --r=200 --c=200` | 62.2 ± 17.8 | 38.9 | 106.9 | 82.88 ± 26.25 |
| `python python_numpy/matmul.py 200 200` | 91.5 ± 7.5 | 83.2 | 121.5 | 121.85 ± 19.24 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 200 200` | 185.1 ± 3.8 | 179.6 | 193.2 | 246.52 ± 33.73 |
| `./rust_peroxide/target/release/rust_peroxide 300 300` | 5.0 ± 0.3 | 4.1 | 5.7 | 6.70 ± 1.01 |
| `./rust_o3/target/release/rust_o3 300 300` | 9.5 ± 2.2 | 6.0 | 31.4 | 12.60 ± 3.36 |
| `./cpp_eigen3/bin/matmul 300 300` | 6.8 ± 0.5 | 6.0 | 8.6 | 9.08 ± 1.39 |
| `./chapel/bin/matmul --r=300 --c=300` | 66.0 ± 18.4 | 41.6 | 105.8 | 87.87 ± 27.24 |
| `python python_numpy/matmul.py 300 300` | 95.0 ± 6.9 | 87.4 | 118.1 | 126.47 ± 19.41 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 300 300` | 194.9 ± 22.1 | 184.3 | 273.8 | 259.59 ± 45.79 |
| `./rust_peroxide/target/release/rust_peroxide 400 400` | 9.2 ± 0.7 | 7.7 | 10.6 | 12.28 ± 1.89 |
| `./rust_o3/target/release/rust_o3 400 400` | 15.5 ± 6.4 | 9.8 | 71.2 | 20.68 ± 9.02 |
| `./cpp_eigen3/bin/matmul 400 400` | 14.7 ± 0.8 | 12.3 | 16.6 | 19.58 ± 2.87 |
| `./chapel/bin/matmul --r=400 --c=400` | 72.4 ± 23.3 | 40.5 | 112.8 | 96.39 ± 33.68 |
| `python python_numpy/matmul.py 400 400` | 100.0 ± 8.9 | 91.2 | 125.4 | 133.13 ± 21.58 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 400 400` | 198.2 ± 20.8 | 187.1 | 272.4 | 263.91 ± 45.15 |
| `./rust_peroxide/target/release/rust_peroxide 500 500` | 15.0 ± 0.9 | 13.0 | 16.5 | 19.96 ± 2.98 |
| `./rust_o3/target/release/rust_o3 500 500` | 23.4 ± 3.3 | 15.6 | 37.6 | 31.19 ± 6.12 |
| `./cpp_eigen3/bin/matmul 500 500` | 25.0 ± 1.5 | 22.0 | 27.8 | 33.31 ± 4.93 |
| `./chapel/bin/matmul --r=500 --c=500` | 83.6 ± 24.5 | 38.3 | 130.6 | 111.37 ± 35.91 |
| `python python_numpy/matmul.py 500 500` | 105.1 ± 8.5 | 91.9 | 127.1 | 139.94 ± 22.06 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 500 500` | 202.1 ± 22.2 | 189.6 | 273.8 | 269.17 ± 46.88 |
| `./rust_peroxide/target/release/rust_peroxide 600 600` | 23.1 ± 1.1 | 20.5 | 25.4 | 30.80 ± 4.43 |
| `./rust_o3/target/release/rust_o3 600 600` | 32.9 ± 6.5 | 21.9 | 65.2 | 43.86 ± 10.44 |
| `./cpp_eigen3/bin/matmul 600 600` | 41.1 ± 1.7 | 37.4 | 43.8 | 54.66 ± 7.72 |
| `./chapel/bin/matmul --r=600 --c=600` | 89.3 ± 27.0 | 38.4 | 146.0 | 118.94 ± 39.43 |
| `python python_numpy/matmul.py 600 600` | 107.8 ± 5.6 | 100.9 | 127.2 | 143.48 ± 20.77 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 600 600` | 210.6 ± 30.2 | 192.1 | 282.9 | 280.43 ± 55.32 |
| `./rust_peroxide/target/release/rust_peroxide 700 700` | 34.3 ± 1.5 | 30.2 | 39.0 | 45.67 ± 6.48 |
| `./rust_o3/target/release/rust_o3 700 700` | 42.7 ± 7.3 | 29.5 | 53.7 | 56.80 ± 12.36 |
| `./cpp_eigen3/bin/matmul 700 700` | 63.0 ± 2.6 | 57.0 | 67.6 | 83.87 ± 11.84 |
| `./chapel/bin/matmul --r=700 --c=700` | 98.7 ± 25.6 | 44.7 | 137.3 | 131.49 ± 38.51 |
| `python python_numpy/matmul.py 700 700` | 114.7 ± 10.0 | 104.4 | 143.7 | 152.68 ± 24.58 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 700 700` | 207.8 ± 22.1 | 196.2 | 283.5 | 276.73 ± 47.63 |
| `./rust_peroxide/target/release/rust_peroxide 800 800` | 50.5 ± 2.5 | 43.4 | 53.7 | 67.27 ± 9.71 |
| `./rust_o3/target/release/rust_o3 800 800` | 56.5 ± 9.7 | 38.7 | 74.6 | 75.18 ± 16.42 |
| `./cpp_eigen3/bin/matmul 800 800` | 90.3 ± 4.2 | 81.1 | 104.5 | 120.23 ± 17.21 |
| `./chapel/bin/matmul --r=800 --c=800` | 115.7 ± 14.8 | 92.3 | 147.2 | 154.09 ± 28.69 |
| `python python_numpy/matmul.py 800 800` | 120.2 ± 7.3 | 108.1 | 139.5 | 160.11 ± 23.76 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 800 800` | 211.5 ± 21.2 | 200.8 | 284.0 | 281.67 ± 47.43 |
| `./rust_peroxide/target/release/rust_peroxide 900 900` | 64.4 ± 2.2 | 59.2 | 69.1 | 85.74 ± 11.96 |
| `./rust_o3/target/release/rust_o3 900 900` | 76.7 ± 13.2 | 49.5 | 133.1 | 102.09 ± 22.36 |
| `./cpp_eigen3/bin/matmul 900 900` | 124.4 ± 5.0 | 115.6 | 131.2 | 165.64 ± 23.36 |
| `./chapel/bin/matmul --r=900 --c=900` | 128.3 ± 16.6 | 98.6 | 157.3 | 170.86 ± 31.95 |
| `python python_numpy/matmul.py 900 900` | 126.1 ± 9.1 | 114.1 | 156.5 | 167.97 ± 25.72 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 900 900` | 209.9 ± 3.0 | 205.6 | 214.1 | 279.53 ± 38.02 |
| `./rust_peroxide/target/release/rust_peroxide 1000 1000` | 88.3 ± 3.9 | 78.3 | 94.2 | 117.64 ± 16.75 |
| `./rust_o3/target/release/rust_o3 1000 1000` | 92.3 ± 13.1 | 63.6 | 112.2 | 122.89 ± 24.11 |
| `./cpp_eigen3/bin/matmul 1000 1000` | 171.9 ± 4.1 | 162.6 | 178.8 | 228.92 ± 31.44 |
| `./chapel/bin/matmul --r=1000 --c=1000` | 135.7 ± 19.6 | 106.8 | 199.5 | 180.74 ± 35.80 |
| `python python_numpy/matmul.py 1000 1000` | 135.6 ± 6.9 | 122.9 | 150.4 | 180.54 ± 26.07 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 1000 1000` | 227.8 ± 24.8 | 213.4 | 299.3 | 303.30 ± 52.69 |
