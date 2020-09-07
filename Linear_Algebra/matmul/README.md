# Matrix multiplcation benchmark

## Hardware specification

* CPU: 6-Core Intel Core i7-9750H (-MT MCP-) speed/min/max: 892/800/4500 MHz 
* Kernel: 5.4.60-2-MANJARO x86_64

## Compilation option

* Rust : `RUSTFLAGS='-C target-cpu=native' cargo build --release`
* Chapel : `chpl -O -I$OPENBLAS_INCLUDE -L$OPENBLAS_LIB -lopenblas`
* C++ : `g++ -I$EIGEN -O2`
* Julia : `julia -O0 --startup-file=no --compile=min`

## Specific version

* Rust : `rustc 1.48.0-nightly`
    * Peroxide : `0.26.0`
* Chapel : `chpl version 1.22.1`
* C++ : `gcc version 10.2.0`
    * Eigen : `3.3.7`
* Julia : `julia version 1.5.0`
* Python : `Python 3.8.5`
    * numpy : `1.19.1`

## Result

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `rust_peroxide/target/release/rust_peroxide` | 0.8 ± 0.1 | 0.7 | 1.3 | 1.00 |
| `./cpp_eigen3/bin/matmul` | 1.2 ± 0.1 | 1.0 | 1.7 | 1.38 ± 0.19 |
| `./chapel/bin/matmul` | 54.2 ± 14.2 | 36.6 | 90.5 | 64.77 ± 18.28 |
| `python python_numpy/matmul.py` | 85.5 ± 2.9 | 81.7 | 93.2 | 102.10 ± 11.13 |
| `julia -O0 --startup-file=no --compile=min julia/matmul.jl` | 128.4 ± 4.0 | 122.5 | 137.4 | 153.36 ± 16.61 |
