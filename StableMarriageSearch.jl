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
function make_preference_functions(M::Matrix, W::Matrix)
    n = size(M, 1)
    @assert size(M,1)==size(M,2)==size(W,1)==size(W,2)

    mp, wp = Dict(), Dict()

    for x ∈ 1:n
        for rank ∈ 1:n
            mp[(x,M[x,rank])]=rank
            wp[(x,W[x,rank])]=rank
        end
    end

    men(man, woman) = mp[(man, woman)]
    women(woman, man) = wp[(woman, man)] 
    return men, women
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
function stability(men::Function, women::Function, matching::Array)
    n = length(matching)
    wife = matching
    husband = Dict(zip(matching, 1:n))
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

using Test
m, w = make_preference_functions([1 2 3; 2 3 1; 3 1 2], [2 3 1; 3 1 2; 1 2 3])
@test stability(m, w, [1, 2, 3]) == 0
@test stability(m, w, [3, 1, 2]) == 0
@test stability(m, w, [2, 3, 1]) == 0
@test stability(m, w, [1, 3, 2]) > 0
@test stability(m, w, [3, 2, 1]) > 0
@test stability(m, w, [2, 1, 3]) > 0


using DataStructures, StatsBase

function informed_search(men, women, matching)
    pq = PriorityQueue()
    visited = Set()
    enqueue!(pq, matching=>stability(men, women, matching))
    while !isempty(pq)
        current = dequeue!(pq)
        push!(visited, current)
        
        if stability(men, women, current) == 0
            println("Found solution: $(current)")
            return current
        else
            println("Explore: $(current)")
        end

        # Create three random permutations from our current solution
        for _ ∈ 1:3
            x = copy(current)
            y = sample(1:length(matching), 2; replace=false)
            x[y[1]], x[y[2]] = x[y[2]], x[y[1]]
            if x ∉ visited && !haskey(pq, x)
                enqueue!(pq, x=>stability(men, women, x))
            end
        end
    end
end;
