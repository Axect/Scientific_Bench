using BenchmarkTools, DataFrames, CSV, LoopVectorization

function sum_avx(x::S) where {T <: Number, S <: AbstractVector{T}}
    s = zero(eltype(x))
    @avx for i in eachindex(x)
        s += x[i]
    end
    return s
end

df = DataFrame(min=Float64[], mean=Float64[], max=Float64[], param=Int64[])

for p in 4:8
    x = 0.0:Float64(10^p);
    b = @benchmark sum_avx($x)
    push!(df, Dict(:min => minimum(b).time, :mean => mean(b).time, :max => maximum(b).time, :param => 10^p))
end

df[:, :min] ./= 10^9
df[:, :mean] ./= 10^9
df[:, :max] ./= 10^9

CSV.write("../bench_julia_avx.csv", df)
