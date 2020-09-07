| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./chapel/bin/matmul` | 54.2 ± 14.2 | 36.6 | 90.5 | 64.77 ± 18.28 |
| `python python_numpy/matmul.py` | 85.5 ± 2.9 | 81.7 | 93.2 | 102.10 ± 11.13 |
| `rust_peroxide/target/release/rust_peroxide` | 0.8 ± 0.1 | 0.7 | 1.3 | 1.00 |
| `./cpp_eigen3/bin/matmul` | 1.2 ± 0.1 | 1.0 | 1.7 | 1.38 ± 0.19 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl` | 128.4 ± 4.0 | 122.5 | 137.4 | 153.36 ± 16.61 |
