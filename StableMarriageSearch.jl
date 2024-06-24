module StableMarriageSearch

export informed_search, stability

using StatsBase, DataStructures, Test

function informed_search(source, edges::Function, heuristic::Function; printgraph=false)
    printgraph && println("digraph {")

    pq = PriorityQueue()
    visited = Set()
    enqueue!(pq, source=>heuristic(source))

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
Determine whether the matchings of a stable marriage problem are stable or not.
The returned integer is a metric quantifying stability.
If the metric is 0, then the matchings are stable.

# Usage
The input matrices must be a "ranking matrix" where the ith row and jth column
show the ranking that man/woman i gives to woman/man j.

The `matching` array is a list pairing men to women, where `matching[i] = j`
relates man `i` to woman `j`.
"""
function stability(men::Matrix, women::Matrix, matching::Vector)
    n = length(matching)
    wife = matching
    husband = Dict(values(matching) .=> keys(matching))
    metric = 0

    for man ∈ 1:n
        for woman ∈ 1:n
            # Candidate and current preferences for the man
            x1, x2 = men[man, woman], men[man, wife[man]]
            # Candidate and current preferences for the woman
            y1, y2 = women[woman, man], women[woman, husband[woman]]
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

"""
Gale and Shapley use a "ranking matrix" where the rows and columns indicate men
and women and the values indicate preference.

Other texts (notably Gusfield and Irving) represent the problem where the 
columns indicate preference (first column is highest preference, second column
is second preference, and so on). Gusfield and Irving use the term "preference
list," so let us call this second format a "preference matrix."

Our stability function requires the ranking matrix format. If a problem is given
in the preference matrix order, then this convenience function changes it to fit
the stability function.
"""
function ranking_matrix(X::Matrix)
    Y = similar(X)
    for (ij,k) ∈ pairs(X)
        (i,j) = Tuple(ij)
        Y[i, k] = j
    end
    return Y
end

@testset "Gale-Shapley" begin
    m = [1 2 3; 2 3 1; 3 1 2]
    w = [2 3 1; 3 1 2; 1 2 3]
    fm, fw = preference_functions(m, w)
    @test stability(fm, fw, [1, 2, 3]) == 0
    @test stability(fm, fw, [3, 1, 2]) == 0
    @test stability(fm, fw, [2, 3, 1]) == 0
    @test stability(fm, fw, [1, 3, 2]) > 0
    @test stability(fm, fw, [3, 2, 1]) > 0
    @test stability(fm, fw, [2, 1, 3]) > 0
    lm, lw = rank_to_lookup(m), rank_to_lookup(w)
    @test stability(lm, lw, [1, 2, 3]) == 0
    @test stability(lm, lw, [3, 1, 2]) == 0
    @test stability(lm, lw, [2, 3, 1]) == 0
    @test stability(lm, lw, [1, 3, 2]) > 0
    @test stability(lm, lw, [3, 2, 1]) > 0
    @test stability(lm, lw, [2, 1, 3]) > 0
end

if !isinteractive()
    using Random
    rng = MersenneTwister(parse(Int, ARGS[1]))

    function e(u)
        v = []
        for _ ∈ 1:3
            x = copy(u)
            y = sample(rng, 1:length(u), 2; replace=false)
            x[y[1]], x[y[2]] = x[y[2]], x[y[1]]
            push!(v, x)
        end
        return v
    end

    rpref(n) = permutedims(reduce(hcat, (shuffle(rng, 1:n) for _ in 1:n)))
    x = parse(Int, ARGS[2])
    mf, wf = preference_functions(rpref(x), rpref(x))
    informed_search(shuffle(rng, 1:x), e, (x) -> stability(mf, wf, x); printgraph=true)
end

end