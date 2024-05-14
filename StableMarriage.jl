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

function isstable2(m, w, a)
    n = length(a)

    mp, wp = Dict(), Dict()

    for x ∈ 1:n
        for rank ∈ 1:n
            mp[(x,m[x,rank])]=rank
            wp[(x,w[x,rank])]=rank
        end
    end

    wife = a
    husband = Dict(zip(a, 1:n))

    for man ∈ 1:n
        for woman ∈ 1:n
            if mp[(man,woman)] < mp[(man, wife[man])] &&
                wp[(woman,man)] < wp[(woman, husband[woman])]
                return false
            end
        end
    end

    true
end

# The Stable Marriage Problem: Structure and Algorithms
# Dan Gusfield and Rubert W. Irving
# https://www.cs.cmu.edu/afs/cs.cmu.edu/academic/class/15251-f10/Site/Materials/Lectures/Lecture21/lecture21.pdf
m = [2 4 1 3; 3 1 4 2; 2 3 1 4; 4 1 3 2]
w = [2 1 4 3; 4 3 1 2; 1 4 3 2; 2 1 4 3]
@test isstable2(m, w, [4, 3, 2, 1]) == true
@test isstable2(m, w, [4, 1, 2, 3]) == true
@test isstable2(m, w, [1, 3, 2, 4]) == false

# https://kostochk.web.illinois.edu/math412-10/Gale-Sh.pdf
m_gale_sh = [1 2 3; 2 3 1; 3 1 2]
w_gale_sh = [2 3 1; 3 1 2; 1 2 3]
@test isstable2(m_gale_sh, w_gale_sh, [1, 2, 3]) == true
@test isstable2(m_gale_sh, w_gale_sh, [3, 1, 2]) == true
@test isstable2(m_gale_sh, w_gale_sh, [2, 3, 1]) == true
@test isstable2(m_gale_sh, w_gale_sh, [1, 3, 2]) == false
@test isstable2(m_gale_sh, w_gale_sh, [3, 2, 1]) == false
@test isstable2(m_gale_sh, w_gale_sh, [2, 1, 3]) == false
