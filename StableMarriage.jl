using Random, Test

rpref(n) = reduce(hcat, [shuffle(1:n) for _ in 1:n])'
rassign(n) = shuffle(1:n)

function isstable(m, w, a)
    wife(man) = a[man]
    husband(woman) = findfirst(==(woman), a)
    
    for man ∈ 1:size(m)[1]
        for woman ∈ m[man,:]
            if woman == wife(man)
                @goto stable_man
            end

            #println("man $(man) is paired with $(wife(man)) but likes woman $(woman):")
            for suitor ∈ w[woman,:]
                #println(" - does woman $(woman) like $(suitor) more than $(husband(woman))?")
                if suitor == husband(woman)
                    #println("woman $(woman) likes her partner $(suitor)")
                    @goto stable_woman
                    #println("unreachable")
                elseif suitor == man
                    #println("Unstable at $(man) $(woman)")
                    return false
                end
            end
            @label stable_woman
        end

        @label stable_man
        #println("done with man $(man)")
    end

    return true
end

# https://www.cs.cmu.edu/afs/cs.cmu.edu/academic/class/15251-f10/Site/Materials/Lectures/Lecture21/lecture21.pdf
m = [2 4 1 3; 3 1 4 2; 2 3 1 4; 4 1 3 2]
w = [2 1 4 3; 4 3 1 2; 1 4 3 2; 2 1 4 3]

@test isstable(m, w, [4, 3, 2, 1]) == true
@test isstable(m, w, [4, 1, 2, 3]) == true
@test isstable(m, w, [1, 3, 2, 4]) == false