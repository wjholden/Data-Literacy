using Random, Test

rpref(n) = permutedims(reduce(hcat, [shuffle(1:n) for _ in 1:n]))
rassign(n) = shuffle(1:n)

function isstable(m::Matrix, w::Matrix, a::Vector)
    wife(man) = a[man]
    husband(woman) = findfirst(==(woman), a)
    
    for man ∈ 1:size(m)[1]
        for woman ∈ m[man,:]
            if woman == wife(man)
                @goto man_is_stable
            end

            for suitor ∈ w[woman,:]
                if suitor == husband(woman)
                    @goto woman_is_stable
                elseif suitor == man
                    return false
                end
            end
            @label woman_is_stable
        end

        @label man_is_stable
    end

    return true
end

# The Stable Marriage Problem: Structure and Algorithms
# Dan Gusfield and Rubert W. Irving
# https://www.cs.cmu.edu/afs/cs.cmu.edu/academic/class/15251-f10/Site/Materials/Lectures/Lecture21/lecture21.pdf
m = [2 4 1 3; 3 1 4 2; 2 3 1 4; 4 1 3 2]
w = [2 1 4 3; 4 3 1 2; 1 4 3 2; 2 1 4 3]
@test isstable(m, w, [4, 3, 2, 1]) == true
@test isstable(m, w, [4, 1, 2, 3]) == true
@test isstable(m, w, [1, 3, 2, 4]) == false

# https://kostochk.web.illinois.edu/math412-10/Gale-Sh.pdf
@test isstable([1 2 3; 2 3 1; 3 1 2], [2 3 1; 3 1 2; 1 2 3], [1, 2, 3]) == true
@test isstable([1 2 3; 2 3 1; 3 1 2], [2 3 1; 3 1 2; 1 2 3], [3, 1, 2]) == true
@test isstable([1 2 3; 2 3 1; 3 1 2], [2 3 1; 3 1 2; 1 2 3], [2, 3, 1]) == true
@test isstable([1 2 3; 2 3 1; 3 1 2], [2 3 1; 3 1 2; 1 2 3], [1, 3, 2]) == false
@test isstable([1 2 3; 2 3 1; 3 1 2], [2 3 1; 3 1 2; 1 2 3], [3, 2, 1]) == false
@test isstable([1 2 3; 2 3 1; 3 1 2], [2 3 1; 3 1 2; 1 2 3], [2, 1, 3]) == false

