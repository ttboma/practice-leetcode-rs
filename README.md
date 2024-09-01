# Table of Content

- [Table of Content](#table-of-content)
- [Introduction](#introduction)
- [Install and Setup](#install-and-setup)
- [Example Usage](#example-usage)
- [API Documentation](#api-documentation)
- [Developer's Note](#developers-note)
  - [Convention](#convention)
  - [How to Use `pre-commit`](#how-to-use-pre-commit)
  - [Visual Studio Code Leverage](#visual-studio-code-leverage)
  - [How to debug with CodeLLDB](#how-to-debug-with-codelldb)
  - [How to debug with lldb](#how-to-debug-with-lldb)
- [TODO](#todo)

# Introduction

Hi,

Traditionally, the first step in learning a new programming language is writing a "Hello, World!" program. Another effective approach, based on my experience, is starting a project that involves solving [LeetCode](https://leetcode.com/) problems. This method not only reinforces fundamental programming concepts but also offers practical experience with real-world challenges.

This repository is a Rust library crate that offers a diverse collection of solutions to various LeetCode problems. By engaging with this project, you will gain practical experience in several key areas:

- **Toolchains, Package Managers, Build Tools, and Cross-Platform Development**:
  - **Toolchains**: Familiarize yourself with the toolchain for each programming language, including its compilers, interpreters, and build systems.
  - **Build Tools**: Leverage tools like Make, CMake, Cargo (for Rust), and others to automate the build and testing processes efficiently.
  - **Cross-Platform Development**: Ensure your code is compatible with various operating systems (Windows, macOS, Linux) by addressing platform-specific issues and dependencies.
  - **Package Management**: Utilize package managers such as npm (for JavaScript), Cargo (for Rust), and pip (for Python) to manage dependencies and simplify development tasks.
  - **Package Creation and Distribution**: Learn how to create, publish, and manage packages or libraries for reuse and distribution.

- **Library and Binary Development**: Develop and oversee a Rust library crate, along with a binary for console applications stored in the `bin` directory. We use the [clap](https://docs.rs/clap/latest/clap/) library for command-line argument parsing and the [nom](https://docs.rs/nom/latest/nom/) library for input parsing.

- **Utilize Git and Git Hooks**: Gain proficiency in Git for version control and explore [Git hooks](https://git-scm.com/book/zh-tw/v2/Customizing-Git-Git-Hooks) to automate tasks within your Rust project. Use the [pre-commit](https://pre-commit.com/) framework to handle code quality hooks across various languages.

- **IDE Utilization with VSCode**: Leverage Visual Studio Code (VSCode) to streamline the development and debugging process for your Rust projects, including setting up and using Rust-specific extensions.

- **Unit Testing**: Utilize Rust's built-in testing features to create and run unit tests, ensuring your code performs as expected and adheres to quality standards.

- **Data Structures and Algorithms**: Deepen your understanding of data structures and algorithms by implementing them in Rust, enhancing both your problem-solving skills and your proficiency in the language.

This project not only helps you tackle LeetCode problems but also provides valuable hands-on experience with essential Rust development tools and practices.

Author: ***SHIEH, YUEH-CHANG***

# Install and Setup

All you need is to install `rustup` from here: [Install Rust](https://www.rust-lang.org/tools/install). Please Read the website to get all prerequisite information you need to know about `rustc`, `cargo`, and `rustup`.

```sh
% rustup --version
rustup 1.27.1 (54dd3d00f 2024-04-24)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.78.0 (9b00956e5 2024-04-29)`
% rustc --version
rustc 1.78.0 (9b00956e5 2024-04-29)
% cargo --version
cargo 1.78.0 (54d8815d0 2024-03-26)
```

# Example Usage

After cloning this library crate using `git clone`, run `cargo test` to execute all unit tests and integration tests.

Here is an example of how to use this library crate in your code:

```rust
use leetcode_rs::Solution;

let ans = Solution::fib(5);
assert_eq!(ans, 5);
```

You can also use the `solution` binary through the command line interface:

```bash
% cargo run --bin solution -- --help
% cargo run --bin solution -- help fib
% echo 10 | cargo run --bin solution -- fib
55
```

# API Documentation

For additional LeetCode solution commands beyond `Solution::fib`, you can view the documentation by running `cargo doc --open`.

Alternatively, you can obtain help through the command line interface with the following commands:

```bash
% cargo run --bin solution -- --help
% cargo run --bin solution -- help <COMMAND>
% cargo run --bin solution -- <COMMAND>
```

# Developer's Note

This section contains essential guidelines to help you contribute effectively and maintain high-quality code. Please adhere to the following conventions and recommendations to ensure consistency and efficiency across our development process.

## Convention

Please follow the conventions, as noted below, to contribute to this project.

- Use chrome and [Clip LeetCode](https://chrome.google.com/webstore/detail/clip-leetcode/cnghimckckgcmhbdokjielmhkmnagdcp/related)
   extension to maintain documentation of each method of [`Solution`]

## How to Use `pre-commit`

Please install [pre-commit](https://pre-commit.com/) with the following command before committing:

```bash
% pre-commit install --hook-type commit-msg --hook-type pre-commit --hook-type pre-push
% pre-commit --version
pre-commit 3.7.1
```

## Visual Studio Code Leverage

To maintain your code quality, the following is a list of recommended [Visual Studio Code](https://code.visualstudio.com/) plugins:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Rust Doc Viewer](https://marketplace.visualstudio.com/items?itemName=JScearcy.rust-doc-viewer)
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- [markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)
- [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)
- [GitLens — Git supercharged](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)
- [pre-commit](https://marketplace.visualstudio.com/items?itemName=elagil.pre-commit-helper)

## How to debug with CodeLLDB

To debug using CodeLLDB, press `ctrl+shift+D` and choose names besides the green triangle button from `Debug unit tests in library 'leetcode_rs'`, `Debug executable 'solution'` and `Debug unit tests in executable 'solution'`, if you got `.vscode/launch.json` as the following.
Note that if you need to pass arguments to binary run by cargo, just change the field `args`

```json
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in library 'leetcode_rs'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--lib",
                    "--package=leetcode_rs"
                ],
                "filter": {
                    "name": "leetcode_rs",
                    "kind": "lib"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'solution'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=solution",
                    "--package=leetcode_rs"
                ],
                "filter": {
                    "name": "solution",
                    "kind": "bin"
                }
            },
            "args": ["fib"], // so that it work like `cargo run --bin=solution -- fib`
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'solution'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=solution",
                    "--package=leetcode_rs"
                ],
                "filter": {
                    "name": "solution",
                    "kind": "bin"
                },
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
    ]
}
```

## How to debug with lldb

For some reason, the CodeLens shortcut `Debug` is not working for binary.
If you need to use lldb on command line:

```bash
$ lldb ./target/debug/solution -- fib
(lldb) target create "./target/debug/solution"
Current executable set to '/Users/shiehyuehchang/Projects/leetcode/leetcode_rs/target/debug/solution' (arm64).
(lldb) settings set -- target.run-args  "fib"
(lldb) breakpoint set -n main
Breakpoint 1: 2 locations.
(lldb) r
Process 85616 launched: '/Users/shiehyuehchang/Projects/leetcode/leetcode_rs/target/debug/solution' (arm64)
Process 85616 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2
    frame #0: 0x00000001000421d0 solution`main
solution`main:
->  0x1000421d0 <+0>:  stp    x29, x30, [sp, #-0x10]!
    0x1000421d4 <+4>:  mov    x29, sp
    0x1000421d8 <+8>:  mov    x2, x1
    0x1000421dc <+12>: mov    x8, x0
Target 0: (solution) stopped.
(lldb) c
Process 85616 resuming
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100027bc4 solution`solution::main::he36a6f7c97382b90 at main.rs:241:9
   238  }
   239
   240  fn main() {
-> 241      let cli = Cli::parse();
   242
   243      // read from file or stdin
   244      let input = if let Some(path) = cli.file.as_deref() {
Target 0: (solution) stopped.
(lldb) n
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100027c98 solution`solution::main::he36a6f7c97382b90 at main.rs:244:37
   241      let cli = Cli::parse();
   242
   243      // read from file or stdin
-> 244      let input = if let Some(path) = cli.file.as_deref() {
   245          read_from_file(path).unwrap()
   246      } else {
   247          read_from_stdin().unwrap()
Target 0: (solution) stopped.
(lldb) n
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100027d24 solution`solution::main::he36a6f7c97382b90 at main.rs:247:9
   244      let input = if let Some(path) = cli.file.as_deref() {
   245          read_from_file(path).unwrap()
   246      } else {
-> 247          read_from_stdin().unwrap()
   248      };
   249
   250      // You can check for the existence of subcommands, and if found use their
Target 0: (solution) stopped.
(lldb) n
10
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100027d58 solution`solution::main::he36a6f7c97382b90 at main.rs:252:11
   249
   250      // You can check for the existence of subcommands, and if found use their
   251      // matches just as you would the top level cmd
-> 252      match &cli.command {
   253          Commands::AllPathsSourceTarget {} => {
   254              let graph = parse_2d_i32_list(&input);
   255              println!("{:?}", Solution::all_paths_source_target(graph));
Target 0: (solution) stopped.
(lldb) n
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100027f28 solution`solution::main::he36a6f7c97382b90 at main.rs:325:31
   322              println!("{:?}", Solution::divisor_game(n));
   323          }
   324          Commands::Fib {} => {
-> 325              let n = parse_i32(&input);
   326              println!("{:?}", Solution::fib(n));
   327          }
   328          Commands::FindBall {} => {
Target 0: (solution) stopped.
(lldb) n
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x000000010002f16c solution`solution::main::he36a6f7c97382b90 at main.rs:326:30
   323          }
   324          Commands::Fib {} => {
   325              let n = parse_i32(&input);
-> 326              println!("{:?}", Solution::fib(n));
   327          }
   328          Commands::FindBall {} => {
   329              let grid = parse_2d_i32_list(&input);
Target 0: (solution) stopped.
(lldb) n
55
Process 85616 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x00000001000288e0 solution`solution::main::he36a6f7c97382b90 at main.rs:746:1
   743              println!("{:?}", Solution::max_area(heights));
   744          }
   745      }
-> 746  }
Target 0: (solution) stopped.
(lldb) q
```

# TODO

- [ ] Refactor nom parsers into builder
