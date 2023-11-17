//! # README
//!
//! Hi SHIEH YUEH-CHANG,
//!
//! The purpose of this project is to practice Rust and DSA.
//!
//! 1. Please run `cargo test` to ensure that all the tests are passed correctly
//! 2. Please run `cargo doc --open` to access the documentation for this crate
//! 3. To access the command line interface for this crate, please run
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
//!
//! ## TODO
//!
//! - [ ] Implement command line program `bin/solution.rs` to be able to use **all** methods of [`Solution`] by `nom` and `clap`
//!     - [ ] Make sure to use [Clip LeetCode](https://chrome.google.com/webstore/detail/clip-leetcode/cnghimckckgcmhbdokjielmhkmnagdcp/related)
//!           for all methods of [`Solution`]
//!     - [ ] ensure that all debug printed output have their proper formatting
//!     - [ ] delete all other binaries (except `bin/solution.rs`)
//!     - [ ] refactor `parse_util` module
//! - [ ] Implement constructor of [`SinglyLinkedList`] by macro

pub use solution::Solution;
mod solution;

pub use tree_node::TreeNode;
mod tree_node;

pub use list_node::ListNode;
pub use list_node::SinglyLinkedList;
mod list_node;

/// utilities
pub mod utils;
