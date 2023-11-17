use clap::{Parser, Subcommand};
use syc_leetcode_solution_rs::utils;
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
    // [894. All Possible Full Binary Trees](https://leetcode.com/problems/all-possible-full-binary-trees/)
    //AllPossibleFbt { n: i32 },
    // [257. Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)
    //binary_tree_paths { root: String },
    /// [605. Can Place Flowers](https://leetcode.com/problems/can-place-flowers/)
    CanPlaceFlowers { flowerbed: String, n: i32 },
    /// [217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)
    ContainsDuplicate { nums: String },
    /// [338. Counting Bits](https://leetcode.com/problems/counting-bits/)
    CountBits { n: i32 },
    /// [2044. Count Number of Maximum Bitwise-OR Subsets](https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/)
    CountMaxOrSubsets { nums: String },
    /// [1523. Count Odd Numbers in an Interval Range](https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/)
    CountOdds { low: i32, high: i32 },
    /// [2316. Count Unreachable Pairs of Nodes in an Undirected Graph](https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/)
    CountPairs { n: i32, edges: String },
    /// [1277. Count Square Submatrices with All Ones](https://leetcode.com/problems/count-square-submatrices-with-all-ones/)
    CountSquares { matrix: String },
    /// [509. Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)
    Fib { n: i32 },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::AllPathsSourceTarget { graph } => {
            let graph = utils::parse_2d_list_i32(graph);
            println!("{:?}", Solution::all_paths_source_target(graph));
        }
        Commands::CanPlaceFlowers { flowerbed, n } => {
            let flowerbed = utils::parse_list_i32(flowerbed);
            println!("{}", Solution::can_place_flowers(flowerbed, *n));
        }
        Commands::ContainsDuplicate { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{}", Solution::contains_duplicate(nums));
        }
        Commands::CountBits { n } => {
            println!("{:?}", Solution::count_bits(*n));
        }
        Commands::CountMaxOrSubsets { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{}", Solution::count_max_or_subsets(nums));
        }
        Commands::CountOdds { low, high } => {
            println!("{}", Solution::count_odds(*low, *high));
        }
        Commands::CountPairs { n, edges } => {
            let edges = utils::parse_2d_list_i32(edges);
            println!("{}", Solution::count_pairs(*n, edges));
        }
        Commands::CountSquares { matrix } => {
            let matrix = utils::parse_2d_list_i32(matrix);
            println!("{}", Solution::count_squares(matrix));
        }
        Commands::Fib { n } => {
            println!("{}", Solution::fib(*n));
        }
    }
}
