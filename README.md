My Advent of Code solutions for 2024. The current goal is to make it through the first 10 days, with solutions in a mix of Rust, C++, Scala, Perl, and R. My intended approach will be to solve each problem naiively (and likely inefficiently) in Rust (only using standard library, no regex!), then attempt to optimize my algorithm before investigating one of the other languages and seeing what options they have for porting a solution.

To use the shell script to generate a Rust template for an answer and unit test for a day's problem, as well as fetch the problem's input, get your session cookie, as shown [here](https://github.com/wimglenn/advent-of-code-wim/issues/1) and place it in a file called `.session` in the main folder. Then, run `./make_day.sh X`, where `X` is the problem's day number.

Benchmarks of different solutions to follow.
