//! # [README](https://github.com/ttboma/leetcode_rs)
//!
//! Hi,
//!
//! This crate is a collection of solutions to [LeetCode](https://leetcode.com/) problems provided by SHIEH YUEH-CHANG.
//! The purpose of this project is to practice Rust and DSA.
//!
//! 1. Please run `cargo test` to ensure that all the tests are passed correctly
//! 2. The following shows an example that how to use this library crate. And please run
//!    `cargo doc --open` to access the documentation.
//!
//!    ```rust
//!    use leetcode_rs::Solution;
//!
//!    let ans = Solution::fib(5);
//!    assert_eq!(ans, 5);
//!    ```
//!
//! 3. To access the command line interface, please run
//!
//!    ```bash
//!    cargo run --bin solution -- --help
//!    cargo run --bin solution -- <COMMAND> --help
//!    cargo run --bin solution -- <COMMAND>
//!    ```
//!
//!    For example:
//!
//!    ```bash
//!    cargo run --bin solution -- --help
//!    cargo run --bin solution -- fib --help
//!    cargo run --bin solution -- fib 5
//!    ```
//!
//! ## Development Note
//!
//! Please follow the rules below to contribute to this project.
//!
//! - Use chrome and [Clip LeetCode](https://chrome.google.com/webstore/detail/clip-leetcode/cnghimckckgcmhbdokjielmhkmnagdcp/related)
//!   extension to maintain documentation of each method of [`Solution`]
//! - Please install the [git hook](https://git-scm.com/book/zh-tw/v2/Customizing-Git-Git-Hooks) scripts
//!   by the following [pre-commit](https://pre-commit.com/) command:
//!
//!   ```bash
//!   pre-commit install --hook-type commit-msg --hook-type pre-commit --hook-type pre-push
//!   ```
//!
//! ## TODO
//!
//! - [ ] ensure that all debug printed output have their proper formatting in `bin/solution.rs`
//!     - [ ] [`TreeNode`] by macro
//!     - [ ] modify [`TreeNode`] unit tests

pub use solution::*;
mod solution;

pub use tree_node::*;
mod tree_node;

pub use list_node::*;
mod list_node;

/// utilities
pub mod utils;
