def fib n = 
    if n <= 1 then
        n
    else
        fib$(n - 1) + fib$(n - 2)
    end
end

print$(fib$(35))
