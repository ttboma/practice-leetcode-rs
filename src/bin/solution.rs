use clap::{Parser, Subcommand};
use leetcode_rs::utils;
use leetcode_rs::Solution;

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
    /// [1638. Count Substrings That Differ by One Character](https://leetcode.com/problems/count-substrings-that-differ-by-one-character/)
    CountSubstrings { s: String, t: String },
    /// [1641. Count Sorted Vowel Strings](https://leetcode.com/problems/count-sorted-vowel-strings/)
    CountVowelStrings { n: i32 },
    /// [394. Decode String](https://leetcode.com/problems/decode-string/)
    DecodeString { s: String },
    /// [241. Different Ways to Add Parentheses](https://leetcode.com/problems/different-ways-to-add-parentheses/)
    DiffWaysToCompute { expression: String },
    /// [2305. Fair Distribution of Cookies](https://leetcode.com/problems/fair-distribution-of-cookies/)
    DistributeCookies { cookies: String, k: i32 },
    /// [1025. Divisor Game](https://leetcode.com/problems/divisor-game/description/)
    DivisorGame { n: i32 },
    /// [509. Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)
    Fib { n: i32 },
    /// [1706. Where Will the Ball Fall](https://leetcode.com/problems/where-will-the-ball-fall/description/)
    FindBall { grid: String },
    /// [1706. Where Will the Ball Fall](https://leetcode.com/problems/where-will-the-ball-fall/description/)
    FindDifference { nums1: String, nums2: String },
    /// [1545. Find Kth Bit in Nth Binary String](https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/)
    FindKthBit { n: i32, k: i32 },
    /// [1539. Kth Missing Positive Number](https://leetcode.com/problems/kth-missing-positive-number/)
    FindKthPositive { arr: String, k: i32 },
    /// [643. Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/)
    FindMaxAverage { nums: String, k: i32 },
    /// [1823. Find the Winner of the Circular Game](https://leetcode.com/problems/find-the-winner-of-the-circular-game/)
    FindTheWinner { n: i32, k: i32 },
    /// [1071. Greatest Common Divisor of Strings](https://leetcode.com/problems/greatest-common-divisor-of-strings/)
    GcdOfStrings { str1: String, str2: String },
    /// [22. Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)
    GenerateParenthesis { n: i32 },
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
            println!("{:?}", Solution::can_place_flowers(flowerbed, *n));
        }
        Commands::ContainsDuplicate { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::contains_duplicate(nums));
        }
        Commands::CountBits { n } => {
            println!("{:?}", Solution::count_bits(*n));
        }
        Commands::CountMaxOrSubsets { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::count_max_or_subsets(nums));
        }
        Commands::CountOdds { low, high } => {
            println!("{:?}", Solution::count_odds(*low, *high));
        }
        Commands::CountPairs { n, edges } => {
            let edges = utils::parse_2d_list_i32(edges);
            println!("{:?}", Solution::count_pairs(*n, edges));
        }
        Commands::CountSquares { matrix } => {
            let matrix = utils::parse_2d_list_i32(matrix);
            println!("{:?}", Solution::count_squares(matrix));
        }
        Commands::CountSubstrings { s, t } => {
            println!("{:?}", Solution::count_substrings(s.clone(), t.clone()));
        }
        Commands::CountVowelStrings { n } => {
            println!("{:?}", Solution::count_vowel_strings(*n));
        }
        Commands::DecodeString { s } => {
            println!("{:?}", Solution::decode_string(s.clone()));
        }
        Commands::DiffWaysToCompute { expression } => {
            println!("{:?}", Solution::diff_ways_to_compute(expression.clone()));
        }
        Commands::DistributeCookies { cookies, k } => {
            let cookies = utils::parse_list_i32(cookies);
            println!("{:?}", Solution::distribute_cookies(cookies, *k));
        }
        Commands::DivisorGame { n } => {
            println!("{:?}", Solution::divisor_game(*n));
        }
        Commands::Fib { n } => {
            println!("{:?}", Solution::fib(*n));
        }
        Commands::FindBall { grid } => {
            let grid = utils::parse_2d_list_i32(grid);
            println!("{:?}", Solution::find_ball(grid));
        }
        Commands::FindDifference { nums1, nums2 } => {
            let nums1 = utils::parse_list_i32(nums1);
            let nums2 = utils::parse_list_i32(nums2);
            println!("{:?}", Solution::find_difference(nums1, nums2));
        }
        Commands::FindKthBit { n, k } => {
            println!("{:?}", Solution::find_kth_bit(*n, *k));
        }
        Commands::FindKthPositive { arr, k } => {
            let arr = utils::parse_list_i32(arr);
            println!("{:?}", Solution::find_kth_positive(arr, *k));
        }
        Commands::FindMaxAverage { nums, k } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::find_max_average(nums, *k));
        }
        Commands::FindTheWinner { n, k } => {
            println!("{:?}", Solution::find_the_winner(*n, *k));
        }
        Commands::GcdOfStrings { str1, str2 } => {
            println!("{:?}", Solution::gcd_of_strings(str1.clone(), str2.clone()));
        }
        Commands::GenerateParenthesis { n } => {
            println!("{:?}", Solution::generate_parenthesis(*n));
        }
    }
}
