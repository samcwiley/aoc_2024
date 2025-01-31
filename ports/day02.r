setwd("/home/sam/Documents/rust/advent_of_code/aoc_2024/src/.inputs/")
filename <- "input02.txt"
input <- readLines(filename) # reads input
# converts list of lines to list of vectors, separating at whitespace
input <- unlist(lapply(input, strsplit, split = " "), recursive = FALSE)
input <- lapply(input, as.numeric) # casting all of the values to numbers

check_safe <- function(report) {
    differences <- diff(report)
    unique_signs <- unique(sign(differences))

    if (length(unique_signs) == 1 && all(abs(differences) >= 1 & abs(differences) <= 3)) {
        return(1)
    }
    return(0)
}


check_safe_dampened <- function(report) {
    safety_threshold <- 3


    for (i in seq_along(report)[-1]) {
        diff_value <- abs(report[i] - report[i - 1])

        # If an unsafe difference is found, try removing a problem report
        if (diff_value > safety_threshold || diff_value == 0) {
            edit_1 <- report[-i]
            edit_2 <- report[-(i - 1)]
            return(check_safe(edit_1) || check_safe(edit_2))
        }

        # Count increasing and decreasing steps
        increasing_count <- sum(diff(report) > 0)
        decreasing_count <- sum(diff(report) < 0)
        increasing <- increasing_count >= decreasing_count

        # If the trend is broken, try removing the problem reports
        if ((increasing && report[i] < report[i - 1]) ||
            (!increasing && report[i] > report[i - 1])) {
            edit_1 <- report[-i]
            edit_2 <- report[-(i - 1)]
            return(check_safe(edit_1) || check_safe(edit_2))
        }
    }

    return(1)
}

# apply check_gradual to each line of input, maps that to a single vector of
# 1s and 0s, and finds the sum
count <- sum(unlist(lapply(input, check_safe)))
print(count)
count <- sum(unlist(lapply(input, check_safe_dampened)))
print(count)
