using DataFrames, CSV, Plots
using Pipe: @pipe
sq = @pipe CSV.read("lifts.csv", DataFrame) |>
    filter(:Lift => ==("Squat")) |>
    rename(_, "Weight (kg)" => :Mass) |>
    select(_, [:Mass, :Repetitions])


fig = plot(scatter(sq.Mass, sq.Repetitions), legend=false)

front = similar(sq, 0)

for x in eachrow(sq)
    for y in eachrow(sq)
        if (x.Mass < y.Mass && x.Repetitions < y.Repetitions) ||
            (x.Mass < y.Mass && x.Repetitions == y.Repetitions) ||
            (x.Mass == y.Mass && x.Repetitions < y.Repetitions)
            @goto not_on_front
        end
    end

    push!(front, x)

    @label not_on_front
end

sort!(front, :Mass)

plot!(fig, front.Mass, front.Repetitions)

@pipe savefig(fig, "pareto.pdf")
