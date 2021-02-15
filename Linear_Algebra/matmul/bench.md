| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./rust_peroxide/target/release/rust_peroxide 100 100` | 0.8 ± 0.1 | 0.5 | 1.2 | 1.00 |
| `./rust_o3/target/release/rust_o3 100 100` | 2.3 ± 0.3 | 2.0 | 6.7 | 2.88 ± 0.57 |
| `./cpp_eigen3/bin/matmul 100 100` | 1.3 ± 0.1 | 1.0 | 1.7 | 1.69 ± 0.27 |
| `python python_numpy/matmul.py 100 100` | 91.4 ± 2.3 | 87.9 | 97.6 | 115.23 ± 14.57 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 100 100` | 118.4 ± 2.4 | 112.1 | 120.9 | 149.35 ± 18.75 |
| `./rust_peroxide/target/release/rust_peroxide 200 200` | 1.9 ± 0.1 | 1.5 | 2.2 | 2.40 ± 0.32 |
| `./rust_o3/target/release/rust_o3 200 200` | 4.0 ± 0.5 | 3.4 | 11.3 | 5.05 ± 0.92 |
| `./cpp_eigen3/bin/matmul 200 200` | 3.2 ± 0.1 | 2.6 | 3.8 | 3.98 ± 0.52 |
| `python python_numpy/matmul.py 200 200` | 93.0 ± 3.9 | 88.3 | 108.3 | 117.35 ± 15.33 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 200 200` | 118.3 ± 4.1 | 112.7 | 131.4 | 149.16 ± 19.20 |
| `./rust_peroxide/target/release/rust_peroxide 300 300` | 4.1 ± 0.2 | 3.3 | 4.6 | 5.11 ± 0.70 |
| `./rust_o3/target/release/rust_o3 300 300` | 7.6 ± 0.7 | 5.9 | 11.8 | 9.53 ± 1.46 |
| `./cpp_eigen3/bin/matmul 300 300` | 6.9 ± 0.4 | 5.9 | 8.1 | 8.68 ± 1.20 |
| `python python_numpy/matmul.py 300 300` | 94.1 ± 3.1 | 90.0 | 101.7 | 118.62 ± 15.20 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 300 300` | 117.8 ± 2.0 | 114.0 | 120.2 | 148.58 ± 18.58 |
| `./rust_peroxide/target/release/rust_peroxide 400 400` | 7.3 ± 0.4 | 6.2 | 9.6 | 9.21 ± 1.23 |
| `./rust_o3/target/release/rust_o3 400 400` | 11.4 ± 0.6 | 9.8 | 13.6 | 14.40 ± 1.93 |
| `./cpp_eigen3/bin/matmul 400 400` | 14.3 ± 0.6 | 12.3 | 15.4 | 18.02 ± 2.37 |
| `python python_numpy/matmul.py 400 400` | 97.4 ± 2.8 | 92.1 | 103.3 | 122.88 ± 15.61 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 400 400` | 120.3 ± 2.9 | 114.2 | 125.7 | 151.78 ± 19.16 |
| `./rust_peroxide/target/release/rust_peroxide 500 500` | 12.5 ± 0.6 | 10.5 | 13.5 | 15.81 ± 2.11 |
| `./rust_o3/target/release/rust_o3 500 500` | 16.7 ± 0.8 | 14.9 | 18.6 | 21.00 ± 2.77 |
| `./cpp_eigen3/bin/matmul 500 500` | 24.9 ± 1.0 | 22.0 | 26.1 | 31.36 ± 4.08 |
| `python python_numpy/matmul.py 500 500` | 99.1 ± 2.6 | 95.8 | 108.9 | 124.95 ± 15.82 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 500 500` | 121.7 ± 2.7 | 117.7 | 128.7 | 153.52 ± 19.33 |
| `./rust_peroxide/target/release/rust_peroxide 600 600` | 19.4 ± 0.8 | 16.9 | 20.6 | 24.51 ± 3.21 |
| `./rust_o3/target/release/rust_o3 600 600` | 23.4 ± 3.6 | 20.7 | 58.9 | 29.51 ± 5.80 |
| `./cpp_eigen3/bin/matmul 600 600` | 39.7 ± 1.1 | 35.7 | 43.6 | 50.01 ± 6.36 |
| `python python_numpy/matmul.py 600 600` | 105.5 ± 4.6 | 100.4 | 121.3 | 133.09 ± 17.47 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 600 600` | 122.6 ± 1.9 | 119.3 | 125.4 | 154.65 ± 19.31 |
| `./rust_peroxide/target/release/rust_peroxide 700 700` | 28.4 ± 1.2 | 25.1 | 31.0 | 35.79 ± 4.70 |
| `./rust_o3/target/release/rust_o3 700 700` | 30.6 ± 0.9 | 27.8 | 32.9 | 38.56 ± 4.91 |
| `./cpp_eigen3/bin/matmul 700 700` | 58.3 ± 1.1 | 54.3 | 61.3 | 73.47 ± 9.21 |
| `python python_numpy/matmul.py 700 700` | 110.4 ± 4.4 | 103.2 | 118.8 | 139.26 ± 18.14 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 700 700` | 128.1 ± 3.0 | 123.0 | 134.1 | 161.54 ± 20.36 |
| `./rust_peroxide/target/release/rust_peroxide 800 800` | 41.0 ± 2.3 | 35.7 | 44.8 | 51.73 ± 7.03 |
| `./rust_o3/target/release/rust_o3 800 800` | 40.1 ± 3.5 | 37.8 | 68.7 | 50.56 ± 7.66 |
| `./cpp_eigen3/bin/matmul 800 800` | 86.5 ± 1.4 | 81.2 | 88.4 | 109.10 ± 13.64 |
| `python python_numpy/matmul.py 800 800` | 116.1 ± 5.3 | 110.5 | 131.2 | 146.48 ± 19.35 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 800 800` | 130.0 ± 2.8 | 124.8 | 136.9 | 164.00 ± 20.62 |
| `./rust_peroxide/target/release/rust_peroxide 900 900` | 55.0 ± 1.6 | 49.7 | 57.5 | 69.38 ± 8.82 |
| `./rust_o3/target/release/rust_o3 900 900` | 50.5 ± 1.2 | 47.8 | 53.5 | 63.67 ± 8.04 |
| `./cpp_eigen3/bin/matmul 900 900` | 123.5 ± 2.4 | 115.9 | 127.6 | 155.74 ± 19.53 |
| `python python_numpy/matmul.py 900 900` | 127.2 ± 4.6 | 118.9 | 133.9 | 160.37 ± 20.71 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 900 900` | 139.7 ± 6.3 | 129.7 | 153.5 | 176.13 ± 23.23 |
| `./rust_peroxide/target/release/rust_peroxide 1000 1000` | 74.7 ± 1.7 | 71.7 | 77.5 | 94.25 ± 11.87 |
| `./rust_o3/target/release/rust_o3 1000 1000` | 65.5 ± 12.9 | 60.5 | 130.9 | 82.63 ± 19.20 |
| `./cpp_eigen3/bin/matmul 1000 1000` | 166.2 ± 1.9 | 158.9 | 168.2 | 209.63 ± 26.08 |
| `python python_numpy/matmul.py 1000 1000` | 133.7 ± 5.5 | 123.2 | 141.0 | 168.58 ± 22.01 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 1000 1000` | 146.0 ± 5.1 | 135.3 | 154.6 | 184.16 ± 23.69 |
