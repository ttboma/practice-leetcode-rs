# [README](https://github.com/ttboma/syc_leetcode_solution_rs)

Hi,

This crate is a collection of solutions to [LeetCode](https://leetcode.com/) problems provided by SHIEH, YUEH-CHANG.
The purpose of this project is to practice Rust and DSA.

1. Please run `cargo test` to ensure that all the tests are passed correctly
2. The following shows an example that how to use this library crate. And please run `cargo doc --open` to access the documentation

   ```rust
   use leetcode_rs::Solution;

   let ans = Solution::fib(5);
   assert_eq!(ans, 5);
   ```

3. To access the command line interface, please run

   ```bash
   $ cargo run --bin solution -- --help
   $ cargo run --bin solution -- <COMMAND> --help
   $ cargo run --bin solution -- <COMMAND>
   ```

   For example:

   ```bash
   $ cargo run --bin solution -- --help
   $ cargo run --bin solution -- fib --help
   $ echo 10 | cargo run --bin solution -- fib
   55
   ```

## Development Note

Please follow the conventions, as noted below, to contribute to this project.

- Use chrome and [Clip LeetCode](https://chrome.google.com/webstore/detail/clip-leetcode/cnghimckckgcmhbdokjielmhkmnagdcp/related)
   extension to maintain documentation of each method of [`Solution`]
- Please install the [git hook](https://git-scm.com/book/zh-tw/v2/Customizing-Git-Git-Hooks) scripts by the following [pre-commit](https://pre-commit.com/) command:

   ```bash
   pre-commit install --hook-type commit-msg --hook-type pre-commit --hook-type pre-push
   ```

It is recommended to use [Visual Studio Code](https://code.visualstudio.com/) and plugins listed in the following to maintain your code quality:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Rust Doc Viewer](https://marketplace.visualstudio.com/items?itemName=JScearcy.rust-doc-viewer)
- [GitLens — Git supercharged](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)
- [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)
- [markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)

## TODO

- [ ] How to implement Iterator for [`TreeNode`]?
- [ ] How to build [`TreeNode`] by macro? And modify those [`TreeNode`] unit tests by macro
