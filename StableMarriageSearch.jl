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

"""
function stability(men::Function, women::Function, matching::Array)
    n = length(matching)
    wife = matching
    husband = Dict(zip(matching, 1:n))
    metric = 0

    for man ∈ 1:n
        for woman ∈ 1:n
            if men(man,woman) < men(man, wife[man]) &&
                women(woman,man) < women(woman, husband[woman])
                metric += (men(man,woman) - men(man, wife[man]))^2
                metric += (women(woman,man) - women(woman, husband[woman]))^2
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
