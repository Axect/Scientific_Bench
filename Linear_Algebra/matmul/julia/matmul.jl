row = parse(Int, ARGS[1])
col = parse(Int, ARGS[2])

function mm(row::Int, col::Int)
    m = rand(row, col)
    n = rand(row, col)
    result = m * n
    println(result[div(row, 2), div(col, 2)])
end

mm(row, col)
