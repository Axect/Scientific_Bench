hyperfine -w 3 \
    --export-markdown bench.md \
    --export-csv bench.csv \
    -P p 4 8 -D 1 \
    "./rust_for/target/release/rust_for {p}" \
    "./rust_simd/target/release/rust_simd {p}" \
    "./rust_chunk/target/release/rust_chunk {p}"
    #"./rust_o3/target/release/rust_o3 {size} {size}" \
    #"./cpp_eigen3_default/bin/matmul {size} {size}" \
    #"./cpp_eigen3_blas/bin/matmul {size} {size}" \
    #"python python_numpy/matmul.py {size} {size}" \
    #"./nim_arraymancer/nim_arraymancer {size} {size}"
#    "./chapel/bin/matmul --r={size} --c={size}" \

