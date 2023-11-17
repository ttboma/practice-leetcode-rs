use clap::{Parser, Subcommand};
use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// [797. All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/)
    AllPathsSourceTarget { graph: String },
    /// [894. All Possible Full Binary Trees](https://leetcode.com/problems/all-possible-full-binary-trees/)
    AllPossibleFbt { n: i32 },
    /// [509. Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)
    Fib { n: i32 },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::AllPathsSourceTarget { graph } => {
            let (_, graph) = parse_util::parse_list_2d(graph).unwrap();
            let graph = graph
                .into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|y| y.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            println!("{:?}", Solution::all_paths_source_target(graph));
        }
        Commands::AllPossibleFbt { n } => {
            println!("{:#?}", Solution::all_possible_fbt(*n));
        }
        Commands::Fib { n } => {
            println!("{}", Solution::fib(*n));
        }
    }
}
