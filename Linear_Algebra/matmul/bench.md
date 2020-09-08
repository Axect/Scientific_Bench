| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./rust_peroxide/target/release/rust_peroxide 100 100` | 0.8 ± 0.1 | 0.6 | 1.5 | 1.00 |
| `./cpp_eigen3/bin/matmul 100 100` | 1.1 ± 0.1 | 1.0 | 1.8 | 1.41 ± 0.21 |
| `./chapel/bin/matmul --r=100 --c=100` | 52.1 ± 13.2 | 37.1 | 87.5 | 64.82 ± 18.18 |
| `python python_numpy/matmul.py 100 100` | 100.1 ± 14.9 | 81.0 | 129.6 | 124.61 ± 23.89 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 100 100` | 136.9 ± 14.5 | 120.0 | 164.5 | 170.38 ± 27.30 |
| `./rust_peroxide/target/release/rust_peroxide 200 200` | 2.1 ± 0.1 | 1.8 | 2.7 | 2.62 ± 0.35 |
| `./cpp_eigen3/bin/matmul 200 200` | 2.9 ± 0.2 | 2.5 | 4.2 | 3.59 ± 0.49 |
| `./chapel/bin/matmul --r=200 --c=200` | 53.0 ± 18.0 | 19.4 | 95.2 | 66.01 ± 23.72 |
| `python python_numpy/matmul.py 200 200` | 87.7 ± 3.6 | 83.0 | 94.6 | 109.21 ± 13.88 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 200 200` | 123.4 ± 2.1 | 120.4 | 128.0 | 153.63 ± 18.66 |
| `./rust_peroxide/target/release/rust_peroxide 300 300` | 4.6 ± 0.3 | 4.1 | 5.5 | 5.73 ± 0.76 |
| `./cpp_eigen3/bin/matmul 300 300` | 6.4 ± 0.3 | 5.9 | 7.5 | 8.00 ± 1.03 |
| `./chapel/bin/matmul --r=300 --c=300` | 61.6 ± 20.6 | 37.9 | 101.6 | 76.67 ± 27.21 |
| `python python_numpy/matmul.py 300 300` | 91.5 ± 3.6 | 87.1 | 101.1 | 113.94 ± 14.41 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 300 300` | 128.5 ± 3.0 | 124.1 | 133.6 | 159.89 ± 19.60 |
| `./rust_peroxide/target/release/rust_peroxide 400 400` | 8.7 ± 0.6 | 7.6 | 9.7 | 10.89 ± 1.48 |
| `./cpp_eigen3/bin/matmul 400 400` | 13.9 ± 0.5 | 12.0 | 16.3 | 17.35 ± 2.18 |
| `./chapel/bin/matmul --r=400 --c=400` | 67.6 ± 18.1 | 39.3 | 101.6 | 84.15 ± 24.67 |
| `python python_numpy/matmul.py 400 400` | 95.7 ± 3.9 | 89.7 | 104.7 | 119.13 ± 15.15 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 400 400` | 132.0 ± 3.9 | 125.0 | 140.2 | 164.33 ± 20.35 |
| `./rust_peroxide/target/release/rust_peroxide 500 500` | 14.9 ± 1.0 | 12.9 | 16.4 | 18.56 ± 2.53 |
| `./cpp_eigen3/bin/matmul 500 500` | 25.0 ± 0.9 | 21.9 | 25.7 | 31.05 ± 3.89 |
| `./chapel/bin/matmul --r=500 --c=500` | 75.1 ± 21.7 | 38.3 | 112.1 | 93.42 ± 29.22 |
| `python python_numpy/matmul.py 500 500` | 100.7 ± 4.9 | 94.4 | 114.5 | 125.28 ± 16.27 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 500 500` | 138.2 ± 4.3 | 130.7 | 147.7 | 171.97 ± 21.39 |
| `./rust_peroxide/target/release/rust_peroxide 600 600` | 23.2 ± 1.5 | 20.4 | 29.9 | 28.93 ± 3.93 |
| `./cpp_eigen3/bin/matmul 600 600` | 38.9 ± 1.8 | 35.1 | 40.8 | 48.45 ± 6.23 |
| `./chapel/bin/matmul --r=600 --c=600` | 73.0 ± 25.0 | 38.6 | 129.1 | 90.81 ± 33.03 |
| `python python_numpy/matmul.py 600 600` | 103.7 ± 3.3 | 98.1 | 108.8 | 129.05 ± 16.05 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 600 600` | 139.4 ± 8.1 | 133.5 | 170.0 | 173.52 ± 23.18 |
| `./rust_peroxide/target/release/rust_peroxide 700 700` | 33.9 ± 1.3 | 30.1 | 36.9 | 42.16 ± 5.33 |
| `./cpp_eigen3/bin/matmul 700 700` | 60.1 ± 0.8 | 55.9 | 60.9 | 74.82 ± 9.06 |
| `./chapel/bin/matmul --r=700 --c=700` | 89.2 ± 20.3 | 48.1 | 126.7 | 111.04 ± 28.62 |
| `python python_numpy/matmul.py 700 700` | 107.9 ± 4.1 | 103.8 | 119.0 | 134.26 ± 16.94 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 700 700` | 139.4 ± 5.0 | 133.7 | 155.5 | 173.48 ± 21.78 |
| `./rust_peroxide/target/release/rust_peroxide 800 800` | 50.1 ± 1.2 | 44.4 | 52.2 | 62.36 ± 7.66 |
| `./cpp_eigen3/bin/matmul 800 800` | 88.4 ± 2.1 | 82.4 | 90.2 | 109.98 ± 13.49 |
| `./chapel/bin/matmul --r=800 --c=800` | 103.6 ± 22.3 | 62.8 | 137.7 | 128.93 ± 31.78 |
| `python python_numpy/matmul.py 800 800` | 118.3 ± 6.5 | 109.8 | 132.2 | 147.30 ± 19.46 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 800 800` | 146.1 ± 5.7 | 137.7 | 158.1 | 181.81 ± 23.01 |
| `./rust_peroxide/target/release/rust_peroxide 900 900` | 66.8 ± 1.6 | 60.1 | 70.3 | 83.11 ± 10.19 |
| `./cpp_eigen3/bin/matmul 900 900` | 121.3 ± 4.1 | 112.4 | 124.9 | 150.99 ± 18.86 |
| `./chapel/bin/matmul --r=900 --c=900` | 113.7 ± 18.2 | 80.4 | 142.2 | 141.46 ± 28.38 |
| `python python_numpy/matmul.py 900 900` | 127.4 ± 5.4 | 117.3 | 135.4 | 158.55 ± 20.23 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 900 900` | 151.1 ± 5.0 | 144.1 | 159.7 | 188.04 ± 23.45 |
| `./rust_peroxide/target/release/rust_peroxide 1000 1000` | 84.7 ± 1.4 | 80.3 | 87.6 | 105.47 ± 12.81 |
| `./cpp_eigen3/bin/matmul 1000 1000` | 161.4 ± 1.3 | 157.9 | 163.5 | 200.86 ± 24.22 |
| `./chapel/bin/matmul --r=1000 --c=1000` | 119.9 ± 16.6 | 66.4 | 142.6 | 149.21 ± 27.37 |
| `python python_numpy/matmul.py 1000 1000` | 133.3 ± 7.0 | 121.1 | 145.3 | 165.88 ± 21.80 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl 1000 1000` | 157.2 ± 7.4 | 147.1 | 173.5 | 195.61 ± 25.28 |
