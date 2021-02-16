using BenchmarkTools, DataFrames, CSV

#row = parse(Int, ARGS[1])
#col = parse(Int, ARGS[2])

function mm(row::Int, col::Int)
    m = rand(row, col)
    n = rand(row, col)
    result = m * n
    return result[div(row, 2), div(col, 2)]
end

df = DataFrame(min=Float64[], mean=Float64[], max=Float64[], param=Int[])

for row in 100:100:1000
    b = @benchmark mm($row, $row)
    push!(df, Dict(:min => minimum(b).time, :mean => mean(b).time, :max => maximum(b).time, :param => row))
end

df[:, :min] ./= 10^9
df[:, :mean] ./= 10^9
df[:, :max] ./= 10^9

CSV.write("../bench_julia.csv", df)
