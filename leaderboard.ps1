(1..10) | ForEach-Object {
    $page = $_;
    (curl "https://c3po.crossfit.com/api/leaderboards/v2/competitions/open/2024/leaderboards?view=0&division=1&region=0&scaled=0&sort=0&page=$($page)" |
    ConvertFrom-Json).leaderboardrows |
    Where-Object {
        $_.entrant.weight -like "*lb" -and $_.entrant.Height -like "*in"
    } |
    ForEach-Object {
        [PSCustomObject]@{
            Name = $_.entrant.competitorName
            Age         = $_.entrant.age
            Weight      = $_.entrant.weight -replace "[^0-9]" , "" # Remove pounds
            Height      = $_.entrant.height -replace "[^0-9]" , "" # Remove inches
            "24.1 Time" = $_.scores[0].time
            "24.2 Reps" = $_.scores[1].score / 10000 # Scores may contain seconds as a tiebreaker
            "24.3 Time" = $_.scores[2].time
        }
    }
} | Export-Csv "2024 open.csv"

# Now bring this into R with:
# df <- read.csv("~/GitHub/Data-Literacy/2024 open.csv", row.names=1)
# library(tidyverse)
# df <- drop_na(df)
# pc <- prcomp(df)
# summary(pc)
# biplot(pc, cex=c(0.5,1))
