using BenchmarkTools, DataFrames, CSV

function sum_for(x::S) where {T <: Number, S <: AbstractVector{T}}
    s = zero(eltype(x))
    for t in x
        s += t
    end
    return s
end

df = DataFrame(min=Float64[], mean=Float64[], max=Float64[], param=Int64[])

for p in 4:8
    b = @benchmark sum_for(0.0:Float64(10^$p))
    push!(df, Dict(:min => minimum(b).time, :mean => mean(b).time, :max => maximum(b).time, :param => 10^p))
end

df[:, :min] ./= 10^9
df[:, :mean] ./= 10^9
df[:, :max] ./= 10^9

CSV.write("../bench_julia_for.csv", df)
