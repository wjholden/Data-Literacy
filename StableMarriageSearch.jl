using Random, StatsBase, DataStructures, Test
rng = MersenneTwister(parse(Int, ARGS[1]))

function informed_search(souce, edges::Function, heuristic::Function; printgraph=false)
    printgraph && println("digraph {")

    pq = PriorityQueue()
    visited = Set()
    enqueue!(pq, souce=>heuristic(souce))

    while !isempty(pq)
        u = dequeue!(pq)
        push!(visited, u)
        printgraph && println("\"$(u)\" [color=\"blue\"];")
        
        if heuristic(u) == 0
            printgraph && println("}")
            return u
        end

        for v ∈ edges(u)
            if v ∉ visited && !haskey(pq, v)
                enqueue!(pq, v=>heuristic(v))
                printgraph && println("\"$(u)\" -> \"$(v)\";")
            end
        end
    end

    error("Failed to find a solution.")
end

"""
Determine whether the matchings of a stable marriage problem are stable or not.
The returned integer is a metric quantifying stability.
If the metric is 0, then the matchings are stable.

# Usage
Generate the `men` and `women` functions using `make_preference_functions`.

The `matching` array is a list pairing men to women, where `matching[i] = j`
relates man `i` to woman `j`.
"""
function stability(men::Function, women::Function, matching::Vector)
    n = length(matching)
    wife = matching
    husband = Dict(values(matching) .=> keys(matching))
    metric = 0

    for man ∈ 1:n
        for woman ∈ 1:n
            # Candidate and current preferences for the man
            x1, x2 = men(man, woman), men(man, wife[man])
            # Candidate and current preferences for the woman
            y1, y2 = women(woman, man), women(woman, husband[woman])
            # The matching is unstable if, and only if, both the man
            # and the woman prefer each other to their current matches.
            if x1 < x2 && y1 < y2
                metric += (x1 - x2)^2
                metric += (y1 - y2)^2
            end
        end
    end

    return metric
end

"""
Create two *preference functions* to relate men and women to their preferences.
For the men's preference matrix, `M`, each row represents one man, columns
indicate the preference from most to least desire, and values indicate the
women. The women's preference matrix, `W`, is the reverse.

# Example

Consider the matrices given by Gale and Shapley,

```
julia> M = [1 2 3; 2 3 1; 3 1 2]
3×3 Matrix{Int64}:
 1  2  3
 2  3  1
 3  1  2

julia> W = [2 3 1; 3 1 2; 1 2 3]
3×3 Matrix{Int64}:
 2  3  1
 3  1  2
 1  2  3
```

`M[2][1] = 2` is the second man's first preference, which is woman two.

`W[1][2] = 3` is the first woman's second preference, which is man three.

The output is a tuple of binary functions. Call them `mpf` and `wpf`.
Continuing the example, `mpf(2, 1) == 2` and `wpf(1, 2) == 3`.
"""
function preference_functions(M::Matrix, W::Matrix)
    n = size(M, 1)
    @assert size(M,1)==size(M,2)==size(W,1)==size(W,2)

    mp, wp = Dict(), Dict()

    for x ∈ 1:n
        for rank ∈ 1:n
            mp[(x,M[x,rank])]=rank
            wp[(x,W[x,rank])]=rank
        end
    end

    return (x,y) -> mp[(x,y)], (y,x) -> wp[(y,x)]
end

let
    m, w = preference_functions([1 2 3; 2 3 1; 3 1 2], [2 3 1; 3 1 2; 1 2 3])
    @test stability(m, w, [1, 2, 3]) == 0
    @test stability(m, w, [3, 1, 2]) == 0
    @test stability(m, w, [2, 3, 1]) == 0
    @test stability(m, w, [1, 3, 2]) > 0
    @test stability(m, w, [3, 2, 1]) > 0
    @test stability(m, w, [2, 1, 3]) > 0
end

function edges(matching)
    chnl = Channel() do ch
        for _ ∈ 1:3
            x = copy(matching)
            y = sample(rng, 1:length(matching), 2; replace=false)
            x[y[1]], x[y[2]] = x[y[2]], x[y[1]]
            push!(ch, x)
        end
    end
    chnl
end

rpref(n) = permutedims(reduce(hcat, (shuffle(rng, 1:n) for _ in 1:n)))
x = parse(Int, ARGS[2])
mf, wf = preference_functions(rpref(x), rpref(x))
informed_search(shuffle(rng, 1:x), edges, (x) -> stability(mf, wf, x); printgraph=true)
