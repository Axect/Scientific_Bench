using BenchmarkTools, DataFrames, CSV, LoopVectorization

function sum_avx(x::S) where {T <: Number, S <: AbstractVector{T}}
    s = zero(eltype(x))
    @avx for i in eachindex(x)
        s += x[i]
    end
    return s
end

df = DataFrame(min=Float64[], mean=Float64[], max=Float64[], param=Int64[])

small_range = [collect(1.0:Float64(p)) for p in 10000:10000:10_0000]
large_range = [collect(1.0:Float64(p)) for p in 1000_0000:1000_0000:10000_0000]
range = vcat(small_range, large_range)

for x in range 
    b = @benchmark sum_avx($x)
    push!(df, Dict(:min => minimum(b).time, :mean => mean(b).time, :max => maximum(b).time, :param => x[end]))
end

df[:, :min] ./= 10^9
df[:, :mean] ./= 10^9
df[:, :max] ./= 10^9

CSV.write("../bench_julia_avx.csv", df)
