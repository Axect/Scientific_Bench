# Scientific Benchmark

## Languages & Libraries

### Rust

* [Peroxide](https://github.com/Axect/Peroxide)

### Python

* [Numpy](https://numpy.org/)
* [Scipy](https://www.scipy.org/)

### C++

* [Eigen3](http://eigen.tuxfamily.org/index.php?title=Main_Page)

### Julia

* Native julia

### Nim

* [Arraymancer](https://github.com/mratsim/Arraymancer)

### Candidates

* Rust    
    * [ndarray](https://github.com/rust-ndarray/ndarray)
    * [nalgebra](https://nalgebra.org/)
* C++
    * [Armadillo](http://arma.sourceforge.net/)

## Benchmark tool & command

Library | Purpose | Command
:-----: | :-----: | :-----:
[Hyperfine](https://github.com/sharkdp/hyperfine) | Benchmark binaries (Nim, C++, Rust) | `hyperfine -w 3 --export-markdown bench.md ${BINARY_FILE}`
[Criterion](https://github.com/bheisler/criterion.rs) | Benchmark rust | `cargo bench`
[BenchmarkTools](https://github.com/JuliaCI/BenchmarkTools.jl) | For julia benchmark | `@benchmark`
[pytest-benchmark](https://github.com/ionelmc/pytest-benchmark) | For python benchmark | `pytest --benchmark-only ${SOURCE_FILE.py} --benchmark-save=bench`

## Computations

### Basic

* [Summation](./Basic/sum)

### Linear Algebra

* [Matrix Multiplication](./Linear_Algebra/matmul)

### IO

* [Database](./IO/db)

### Numerical Computations

### Machine Learning

### Physics
