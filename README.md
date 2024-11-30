My rust workspace for [advent of code 2024](https://adventofcode.com/2024). Every solution could be improved upon, but should at least be readable; this was written mostly as a learning opportunity. I'm putting it out there to expose the process of learning.

I recommend attempting to solve the puzzles before looking at the solutions.

# Building

The inputs are built-in and you need to have them in `src/inputs` in order to build the project. Otherwise you can use the CI feature and use the samples instead of real inputs:

```sh
# If you put your inputs in src/inputs/
cargo build --release
# Otherwise, just use the samples
cargo build --release --features ci_no_input
```
