function bench()

    m = zeros(100, 100)
    
    for i = 0:99
        for j = 0:99
            m[i+1,j+1] = i*100 + j
        end
    end
    
    n = m
    
    result = m * n
    
    println(result[div(100, 2)+1, div(100, 2)+1])

end

@time bench()
