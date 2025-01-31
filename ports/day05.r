setwd("/home/sam/Documents/rust/advent_of_code/aoc_2024/src/.inputs/")
filename <- "input05.txt"
input <- readLines(filename)

# finding lines with | and splitting them into pairs of integers
# (\\ is just to escape so the pipe doesn't get interpreted as regex)
# sapply coerces the output into a vector
ordered_pairs <- sapply(strsplit(input[grep("\\|", input)], "\\|"), as.integer)
# updates needs to be a list because each set of updates can be variable length
updates <- lapply(strsplit(input[grep(",", input)], ","), as.integer)

# Function to check order consistency
check_order <- function(x) {
    keep_columns <- apply(ordered_pairs, 2, function(pair) {
        count <- sum(x %in% pair) # Count how many elements of x are in this pair
        return(count == 2)
    })

    filtered_pairs <- ordered_pairs[, keep_columns]

    # Sort x based on the first element of the filtered pairs
    # table() creates a frequency count of elements in first row of filtered_pairs
    # we sort based on frequencies (ascending because we negated table)
    # we get the names (like hash keys) and cast them back to integers
    sorted_x <- as.integer(names(sort(-table(filtered_pairs[1, ]))))

    # Append missing elements from x (makes sure we didn't drop any elements
    # that weren't included in the rules)
    sorted_x <- c(sorted_x, setdiff(x, filtered_pairs[1, ]))

    # Return a signed value based on order consistency
    middle_index <- (length(x) + 1) / 2
    # if the sorted list is the same as the original, we get positive value, otherwise negative
    sign_value <- if (identical(sorted_x, x)) 1 else -1

    return(sorted_x[middle_index] * sign_value)
}

# Apply check_order function to each update
results <- sapply(updates, check_order)

# Print results
cat("Part One:", sum(results[results > 0]), "\n")
cat("Part Two:", sum(-results[results < 0]), "\n")
