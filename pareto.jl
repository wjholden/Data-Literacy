using DataFrames, CSV, Plots
using Pipe: @pipe
sq = @pipe CSV.read("lifts.csv", DataFrame) |>
    filter(:Lift => ==("Squat")) |>
    rename(_, "Weight (kg)" => :Mass) |>
    select(_, [:Mass, :Repetitions])


fig = plot(scatter(sq.Mass, sq.Repetitions), legend=false)

front = similar(sq, 0)

for x in eachrow(sq)
    on_front = true
    for y in eachrow(sq)
        if (x.Mass < y.Mass && x.Repetitions < y.Repetitions) ||
            (x.Mass < y.Mass && x.Repetitions == y.Repetitions) ||
            (x.Mass == y.Mass && x.Repetitions < y.Repetitions)
            on_front = false
            break
        end
    end
    if on_front
        push!(front, x)
    end
end

sort!(front, :Mass)

plot!(fig, front.Mass, front.Repetitions)

@pipe savefig(fig, "pareto.pdf")

