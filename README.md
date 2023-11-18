# [README](https://github.com/ttboma/syc_leetcode_solution_rs)

Hi,

The purpose of this project is to practice Rust and DSA.

1. Please run `cargo test` to ensure that all the tests are passed correctly
2. The following shows an example that how to use this library crate, to access
   the documentation, please run `cargo doc --open`

   ```rust
   use leetcode_rs::Solution;

   let ans = Solution::fib(5);
   assert_eq!(ans, 5);
   ```

3. To access the command line interface, please run

   ```bash
   cargo run --bin solution -- --help
   cargo run --bin solution -- <COMMAND> --help
   cargo run --bin solution -- <COMMAND>
   ```

   For example:

   ```bash
   cargo run --bin solution -- --help
   cargo run --bin solution -- fib --help
   cargo run --bin solution -- fib 5
   ```
