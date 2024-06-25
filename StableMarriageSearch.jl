module StableMarriageSearch

export informed_search, stability

using DataStructures

function informed_search(source, edges::Function, heuristic::Function; printdot=false)
    printdot && println("digraph {")

    pq = PriorityQueue()
    visited = Set()
    enqueue!(pq, source=>heuristic(source))

    while !isempty(pq)
        u = dequeue!(pq)
        push!(visited, u)
        printdot && println("\"$(u)\" [color=\"blue\"];")
        
        if heuristic(u) == 0
            printdot && println("}")
            return u
        end

        for v ∈ edges(u)
            if v ∉ visited && !haskey(pq, v)
                enqueue!(pq, v=>heuristic(v))
                printdot && println("\"$(u)\" -> \"$(v)\";")
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
The input matrices must be a "ranking matrix" where the ith row and jth column
show the ranking that man/woman i gives to woman/man j.

The `matching` array is a list pairing men to women, where `matching[i] = j`
relates man `i` to woman `j`.
"""
function stability(men::Matrix, women::Matrix, matching)
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

using Test

function verify_ranking_matrix(R::Matrix)
    @testset "Ranking Matrix" begin
        n = size(R,1)
        @test n == size(R, 2)

        @testset "Distinct elements in each row" for row in eachrow(R)
            @test allunique(row)
        end

        @testset "Counting numbers 1 to n" begin
            @test all(>(0), R)
            @test all(≤(n), R)
        end

        @test sum(R) == n * n * (n+1) / 2
    end;
    true
end

@testset "Gale-Shapley example 1" begin
    # https://kostochk.web.illinois.edu/math412-10/Gale-Sh.pdf
    pm = [1 2 3; 2 3 1; 3 1 2]
    pw = [2 3 1; 3 1 2; 1 2 3]
    # If you look very closely then you will see that the above inputs are
    # preference matrices and not ranking matrices. We use a convenience
    # function to translate the input to the expected format.
    lm, lw = ranking_matrix(pm), ranking_matrix(pw)
    @test lm == [1 2 3; 3 1 2; 2 3 1]
    @test lw == [3 1 2; 2 3 1; 1 2 3]
    @test verify_ranking_matrix(lm)
    @test verify_ranking_matrix(lw)
    @test stability(lm, lw, [1, 2, 3]) == 0
    @test stability(lm, lw, [3, 1, 2]) == 0
    @test stability(lm, lw, [2, 3, 1]) == 0
    @test stability(lm, lw, [1, 3, 2]) > 0
    @test stability(lm, lw, [3, 2, 1]) > 0
    @test stability(lm, lw, [2, 1, 3]) > 0
end

if !isinteractive()
    if length(ARGS) != 2
        error("Usage: julia StableMarriageSearch.jl [rng-seed (Int)] [pair-count (Int)")
    end

    using Random, StatsBase
    rng = MersenneTwister(parse(Int, ARGS[1]))

    function e(u)
        v = Set()
        for _ ∈ 1:3
            x = copy(u)
            # Sample, without replacement, two random elements to swap.
            y = sample(rng, collect(eachindex(u)), 2, replace=false)
            x[y[1]], x[y[2]] = x[y[2]], x[y[1]]
            push!(v, x)
        end
        return v
    end

    rpref(n) = permutedims(reduce(hcat, (shuffle(rng, 1:n) for _ in 1:n)))
    x = parse(Int, ARGS[2])
    M = rpref(x)
    W = rpref(x)

    verify_ranking_matrix(M)
    verify_ranking_matrix(W)

    informed_search(shuffle(rng, 1:x), e, (u) -> stability(M, W, u); printdot=true)
end

end