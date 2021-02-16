| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./rust_peroxide/target/release/rust_peroxide 100 100` | 0.8 ± 0.1 | 0.6 | 1.2 | 1.00 |
| `./rust_o3/target/release/rust_o3 100 100` | 2.5 ± 1.8 | 2.2 | 44.8 | 3.26 ± 2.40 |
| `./cpp_eigen3_default/bin/matmul 100 100` | 1.2 ± 0.2 | 1.0 | 3.5 | 1.58 ± 0.27 |
| `./cpp_eigen3_blas/bin/matmul 100 100` | 3.0 ± 0.9 | 2.4 | 18.4 | 3.82 ± 1.26 |
| `python python_numpy/matmul.py 100 100` | 88.2 ± 2.1 | 85.4 | 92.9 | 114.19 ± 13.02 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 100 100` | 114.7 ± 2.0 | 111.0 | 118.0 | 148.45 ± 16.76 |
| `./nim_arraymancer/nim_arraymancer 100 100` | 2.7 ± 0.5 | 2.4 | 10.3 | 3.50 ± 0.75 |
| `./rust_peroxide/target/release/rust_peroxide 200 200` | 1.8 ± 0.1 | 1.5 | 2.2 | 2.34 ± 0.32 |
| `./rust_o3/target/release/rust_o3 200 200` | 4.5 ± 2.9 | 3.2 | 59.3 | 5.83 ± 3.84 |
| `./cpp_eigen3_default/bin/matmul 200 200` | 3.1 ± 0.3 | 2.6 | 3.8 | 4.06 ± 0.57 |
| `./cpp_eigen3_blas/bin/matmul 200 200` | 3.4 ± 0.3 | 3.2 | 6.9 | 4.46 ± 0.62 |
| `python python_numpy/matmul.py 200 200` | 94.0 ± 2.5 | 90.8 | 101.1 | 121.67 ± 13.97 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 200 200` | 119.7 ± 1.6 | 117.3 | 124.2 | 154.97 ± 17.42 |
| `./nim_arraymancer/nim_arraymancer 200 200` | 4.1 ± 1.0 | 3.7 | 22.2 | 5.31 ± 1.39 |
| `./rust_peroxide/target/release/rust_peroxide 300 300` | 4.1 ± 0.4 | 3.3 | 4.9 | 5.37 ± 0.77 |
| `./rust_o3/target/release/rust_o3 300 300` | 7.8 ± 0.5 | 6.3 | 11.7 | 10.10 ± 1.31 |
| `./cpp_eigen3_default/bin/matmul 300 300` | 7.1 ± 0.6 | 6.0 | 9.0 | 9.22 ± 1.25 |
| `./cpp_eigen3_blas/bin/matmul 300 300` | 5.0 ± 1.3 | 4.3 | 20.1 | 6.52 ± 1.89 |
| `python python_numpy/matmul.py 300 300` | 98.9 ± 3.6 | 92.3 | 106.7 | 128.02 ± 15.01 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 300 300` | 123.8 ± 4.9 | 117.3 | 139.5 | 160.30 ± 18.99 |
| `./nim_arraymancer/nim_arraymancer 300 300` | 5.6 ± 2.4 | 5.0 | 48.0 | 7.27 ± 3.24 |
| `./rust_peroxide/target/release/rust_peroxide 400 400` | 7.9 ± 0.6 | 6.5 | 9.6 | 10.22 ± 1.40 |
| `./rust_o3/target/release/rust_o3 400 400` | 12.7 ± 1.6 | 10.4 | 27.9 | 16.42 ± 2.75 |
| `./cpp_eigen3_default/bin/matmul 400 400` | 15.4 ± 0.6 | 12.2 | 16.0 | 19.95 ± 2.37 |
| `./cpp_eigen3_blas/bin/matmul 400 400` | 7.3 ± 2.4 | 6.1 | 44.6 | 9.40 ± 3.28 |
| `python python_numpy/matmul.py 400 400` | 101.6 ± 2.7 | 97.3 | 109.0 | 131.50 ± 15.10 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 400 400` | 125.8 ± 3.8 | 119.0 | 131.9 | 162.82 ± 18.83 |
| `./nim_arraymancer/nim_arraymancer 400 400` | 9.0 ± 1.0 | 8.1 | 20.4 | 11.60 ± 1.81 |
| `./rust_peroxide/target/release/rust_peroxide 500 500` | 13.0 ± 0.8 | 10.9 | 14.6 | 16.81 ± 2.15 |
| `./rust_o3/target/release/rust_o3 500 500` | 18.1 ± 0.9 | 16.4 | 20.6 | 23.39 ± 2.84 |
| `./cpp_eigen3_default/bin/matmul 500 500` | 27.5 ± 0.5 | 23.3 | 28.5 | 35.59 ± 4.04 |
| `./cpp_eigen3_blas/bin/matmul 500 500` | 10.3 ± 0.8 | 8.9 | 13.5 | 13.38 ± 1.81 |
| `python python_numpy/matmul.py 500 500` | 107.7 ± 3.7 | 102.7 | 115.5 | 139.38 ± 16.29 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 500 500` | 128.4 ± 4.1 | 121.3 | 134.5 | 166.24 ± 19.30 |
| `./nim_arraymancer/nim_arraymancer 500 500` | 14.0 ± 0.9 | 12.7 | 17.1 | 18.15 ± 2.33 |
| `./rust_peroxide/target/release/rust_peroxide 600 600` | 20.6 ± 1.2 | 16.6 | 22.2 | 26.65 ± 3.33 |
| `./rust_o3/target/release/rust_o3 600 600` | 25.6 ± 4.5 | 23.0 | 66.3 | 33.08 ± 6.89 |
| `./cpp_eigen3_default/bin/matmul 600 600` | 42.4 ± 1.7 | 36.6 | 44.9 | 54.90 ± 6.53 |
| `./cpp_eigen3_blas/bin/matmul 600 600` | 14.2 ± 2.0 | 12.3 | 27.3 | 18.40 ± 3.27 |
| `python python_numpy/matmul.py 600 600` | 111.0 ± 4.1 | 105.6 | 123.8 | 143.72 ± 16.91 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 600 600` | 132.7 ± 11.5 | 123.1 | 183.3 | 171.72 ± 24.27 |
| `./nim_arraymancer/nim_arraymancer 600 600` | 19.9 ± 3.3 | 17.3 | 48.0 | 25.78 ± 5.14 |
| `./rust_peroxide/target/release/rust_peroxide 700 700` | 30.3 ± 1.9 | 25.7 | 33.4 | 39.20 ± 5.00 |
| `./rust_o3/target/release/rust_o3 700 700` | 33.5 ± 1.4 | 30.4 | 36.8 | 43.32 ± 5.16 |
| `./cpp_eigen3_default/bin/matmul 700 700` | 63.5 ± 1.9 | 57.6 | 65.5 | 82.17 ± 9.51 |
| `./cpp_eigen3_blas/bin/matmul 700 700` | 18.8 ± 1.2 | 16.8 | 25.3 | 24.37 ± 3.14 |
| `python python_numpy/matmul.py 700 700` | 115.0 ± 2.9 | 110.7 | 120.8 | 148.89 ± 17.05 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 700 700` | 134.8 ± 2.7 | 128.5 | 139.9 | 174.49 ± 19.80 |
| `./nim_arraymancer/nim_arraymancer 700 700` | 25.1 ± 1.7 | 22.6 | 33.9 | 32.46 ± 4.21 |
| `./rust_peroxide/target/release/rust_peroxide 800 800` | 43.4 ± 1.3 | 38.8 | 45.4 | 56.19 ± 6.49 |
| `./rust_o3/target/release/rust_o3 800 800` | 43.0 ± 1.4 | 39.6 | 47.0 | 55.72 ± 6.48 |
| `./cpp_eigen3_default/bin/matmul 800 800` | 93.6 ± 2.5 | 84.2 | 96.0 | 121.19 ± 13.90 |
| `./cpp_eigen3_blas/bin/matmul 800 800` | 24.3 ± 2.0 | 21.5 | 33.3 | 31.51 ± 4.38 |
| `python python_numpy/matmul.py 800 800` | 121.8 ± 3.7 | 116.0 | 129.4 | 157.61 ± 18.24 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 800 800` | 138.6 ± 6.1 | 133.4 | 158.9 | 179.44 ± 21.54 |
| `./nim_arraymancer/nim_arraymancer 800 800` | 33.5 ± 3.6 | 29.7 | 48.2 | 43.39 ± 6.76 |
| `./rust_peroxide/target/release/rust_peroxide 900 900` | 60.7 ± 3.0 | 51.5 | 66.5 | 78.60 ± 9.61 |
| `./rust_o3/target/release/rust_o3 900 900` | 56.2 ± 9.3 | 51.8 | 120.1 | 72.77 ± 14.51 |
| `./cpp_eigen3_default/bin/matmul 900 900` | 126.3 ± 3.4 | 121.2 | 132.8 | 163.55 ± 18.79 |
| `./cpp_eigen3_blas/bin/matmul 900 900` | 30.9 ± 1.7 | 28.1 | 38.3 | 40.00 ± 4.95 |
| `python python_numpy/matmul.py 900 900` | 130.2 ± 4.5 | 123.5 | 140.6 | 168.60 ± 19.71 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 900 900` | 141.9 ± 3.1 | 136.5 | 145.5 | 183.63 ± 20.88 |
| `./nim_arraymancer/nim_arraymancer 900 900` | 46.5 ± 6.4 | 39.9 | 69.0 | 60.18 ± 10.65 |
| `./rust_peroxide/target/release/rust_peroxide 1000 1000` | 78.0 ± 3.4 | 69.5 | 82.4 | 100.94 ± 12.10 |
| `./rust_o3/target/release/rust_o3 1000 1000` | 67.0 ± 1.6 | 64.4 | 74.7 | 86.70 ± 9.88 |
| `./cpp_eigen3_default/bin/matmul 1000 1000` | 173.1 ± 3.3 | 165.6 | 178.6 | 224.14 ± 25.39 |
| `./cpp_eigen3_blas/bin/matmul 1000 1000` | 37.8 ± 1.7 | 35.0 | 43.2 | 48.98 ± 5.91 |
| `python python_numpy/matmul.py 1000 1000` | 138.8 ± 5.3 | 127.7 | 149.3 | 179.64 ± 21.18 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 1000 1000` | 149.9 ± 4.2 | 139.7 | 154.8 | 194.11 ± 22.34 |
| `./nim_arraymancer/nim_arraymancer 1000 1000` | 49.5 ± 1.6 | 46.9 | 54.5 | 64.10 ± 7.45 |
