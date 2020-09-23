# Matrix multiplcation benchmark

## Hardware specification

* CPU: 6-Core Intel Core i7-9750H (-MT MCP-) speed/min/max: 892/800/4500 MHz 
* Kernel: 5.4.60-2-MANJARO x86_64

## Compilation option

* Rust : `RUSTFLAGS='-C target-cpu=native' cargo build --release`
* Rust(BLAS) : `RUSTFLAGS='-C target-cpu=native' cargo build --release --features O3`
* Chapel : `chpl -O -I$OPENBLAS_INCLUDE -L$OPENBLAS_LIB -lopenblas`
* C++ : `g++ -I$EIGEN -O2`
* Julia : `julia -O0 --startup-file=no --compile=min`

## Specific version

* Rust : `rustc 1.48.0-nightly`
    * Peroxide : `0.26.3`
* Chapel : `chpl version 1.22.1`
* C++ : `gcc version 10.2.0`
    * Eigen : `3.3.7`
* Julia : `julia version 1.5.0`
* Python : `Python 3.8.5`
    * numpy : `1.19.1`

## Result

![Plot](plot.png)
