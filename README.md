## 2024 Advent of Code in *20% Rust*
--------

This is my take on the [2024 Advent of Code](https://adventofcode.com/2024).

According to my credo, ordinary system integrators and application developers like me can accomplish 85% of their tasks with just 20% of Rust (this is obviously not precise science). In the mindset of **20% Rust** I solve the Advent of Code problems with the following constraints:

- I do only so much as is needed to solve the given task (no efforts for generalization).
- reduced error handling: if input is not as described, I will panic.
- I abstain from advanced, difficult, difficult-to-read or esoteric Rust syntax as much as possible. One exception is the nice and sophisticated read_lines function, that I took in slightly modified from [Rust By Example](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html).
- I do not optimize for executable size, performance or memory footprint.
- I stay in safe mode.
- I try to keep the code readable for non-Rustaceans.
- No abstractions, except when it really helps.

Happy Advent season!