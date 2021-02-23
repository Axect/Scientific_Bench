using BenchmarkTools, DataFrames, CSV

function sum_simd(x::S) where {T <: Number, S <: AbstractVector{T}}
    s = zero(eltype(x))
    @inbounds @simd for t in x
        s += t
    end
    return s
end

df = DataFrame(min=Float64[], mean=Float64[], max=Float64[], param=Int64[])

for p in 4:8
    b = @benchmark sum_simd(0.0:Float64(10^$p))
    push!(df, Dict(:min => minimum(b).time, :mean => mean(b).time, :max => maximum(b).time, :param => 10^p))
end

df[:, :min] ./= 10^9
df[:, :mean] ./= 10^9
df[:, :max] ./= 10^9

CSV.write("../bench_julia_simd.csv", df)
