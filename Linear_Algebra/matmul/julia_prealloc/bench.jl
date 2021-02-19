using BenchmarkTools, DataFrames, CSV, Random

#row = parse(Int, ARGS[1])
#col = parse(Int, ARGS[2])

function mm(A::T, B::T) where T <: AbstractMatrix
    rand!(A); rand!(B);
    result = A * B
    return result[end, end]
end

df = DataFrame(min=Float64[], mean=Float64[], max=Float64[], param=Int[])

for row in 100:100:1000
    A = Array{Float64}(undef, row, row)
    B = Array{Float64}(undef, row, row)
    b = @benchmark mm($A, $B)
    push!(df, Dict(:min => minimum(b).time, :mean => mean(b).time, :max => maximum(b).time, :param => row))
end

df[:, :min] ./= 10^9
df[:, :mean] ./= 10^9
df[:, :max] ./= 10^9

CSV.write("../bench_julia_prealloc.csv", df)
