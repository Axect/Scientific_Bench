#using BenchmarkTools

row = parse(Int, ARGS[1])
col = parse(Int, ARGS[2])

function mm(row::Int, col::Int)
    m = rand(row, col)
    n = rand(row, col)
    result = m * n
    return result[div(row, 2), div(col, 2)]
end

mm(row, col)

#@btime mm(row, col)
