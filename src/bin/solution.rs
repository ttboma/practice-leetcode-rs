use clap::{Parser, Subcommand};
use leetcode_rs::utils;
use leetcode_rs::*;

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
    AllPathsSourceTarget {
        graph: String,
    },
    // [894. All Possible Full Binary Trees](https://leetcode.com/problems/all-possible-full-binary-trees/)
    //AllPossibleFbt { n: i32 },
    // [257. Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)
    //binary_tree_paths { root: String },
    /// [605. Can Place Flowers](https://leetcode.com/problems/can-place-flowers/)
    CanPlaceFlowers {
        flowerbed: String,
        n: i32,
    },
    /// [217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)
    ContainsDuplicate {
        nums: String,
    },
    /// [338. Counting Bits](https://leetcode.com/problems/counting-bits/)
    CountBits {
        n: i32,
    },
    /// [2044. Count Number of Maximum Bitwise-OR Subsets](https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/)
    CountMaxOrSubsets {
        nums: String,
    },
    /// [1523. Count Odd Numbers in an Interval Range](https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/)
    CountOdds {
        low: i32,
        high: i32,
    },
    /// [2316. Count Unreachable Pairs of Nodes in an Undirected Graph](https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/)
    CountPairs {
        n: i32,
        edges: String,
    },
    /// [1277. Count Square Submatrices with All Ones](https://leetcode.com/problems/count-square-submatrices-with-all-ones/)
    CountSquares {
        matrix: String,
    },
    /// [1638. Count Substrings That Differ by One Character](https://leetcode.com/problems/count-substrings-that-differ-by-one-character/)
    CountSubstrings {
        s: String,
        t: String,
    },
    /// [1641. Count Sorted Vowel Strings](https://leetcode.com/problems/count-sorted-vowel-strings/)
    CountVowelStrings {
        n: i32,
    },
    /// [394. Decode String](https://leetcode.com/problems/decode-string/)
    DecodeString {
        s: String,
    },
    /// [241. Different Ways to Add Parentheses](https://leetcode.com/problems/different-ways-to-add-parentheses/)
    DiffWaysToCompute {
        expression: String,
    },
    /// [2305. Fair Distribution of Cookies](https://leetcode.com/problems/fair-distribution-of-cookies/)
    DistributeCookies {
        cookies: String,
        k: i32,
    },
    /// [1025. Divisor Game](https://leetcode.com/problems/divisor-game/description/)
    DivisorGame {
        n: i32,
    },
    /// [509. Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)
    Fib {
        n: i32,
    },
    /// [1706. Where Will the Ball Fall](https://leetcode.com/problems/where-will-the-ball-fall/description/)
    FindBall {
        grid: String,
    },
    /// [1706. Where Will the Ball Fall](https://leetcode.com/problems/where-will-the-ball-fall/description/)
    FindDifference {
        nums1: String,
        nums2: String,
    },
    /// [1545. Find Kth Bit in Nth Binary String](https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/)
    FindKthBit {
        n: i32,
        k: i32,
    },
    /// [1539. Kth Missing Positive Number](https://leetcode.com/problems/kth-missing-positive-number/)
    FindKthPositive {
        arr: String,
        k: i32,
    },
    /// [643. Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/)
    FindMaxAverage {
        nums: String,
        k: i32,
    },
    /// [1823. Find the Winner of the Circular Game](https://leetcode.com/problems/find-the-winner-of-the-circular-game/)
    FindTheWinner {
        n: i32,
        k: i32,
    },
    /// [1071. Greatest Common Divisor of Strings](https://leetcode.com/problems/greatest-common-divisor-of-strings/)
    GcdOfStrings {
        str1: String,
        str2: String,
    },
    /// [22. Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)
    GenerateParenthesis {
        n: i32,
    },
    /// [118. Pascal's Triangle](https://leetcode.com/problems/pascals-triangle/)
    Generate {
        num_rows: i32,
    },
    /// [1387. Sort Integers by The Power Value](https://leetcode.com/problems/sort-integers-by-the-power-value/)
    GetKth {
        lo: i32,
        hi: i32,
        k: i32,
    },
    /// [1646. Get Maximum in Generated Array](https://leetcode.com/problems/get-maximum-in-generated-array/description/)
    GetMaximumGenerated {
        n: i32,
    },
    /// [119. Pascal's Triangle II](https://leetcode.com/problems/pascals-triangle-ii/)
    GetRow {
        row_index: i32,
    },
    // [112. Path Sum](https://leetcode.com/problems/path-sum/)
    //HasPathSum{root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32},
    /// [242. Valid Anagram](https://leetcode.com/problems/valid-anagram/)
    IsAnagram {
        s: String,
        t: String,
    },
    /// [205. Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/)
    IsIsomorphic {
        s: String,
        t: String,
    },
    // [234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/description/)
    IsPalindrome {
        head: String,
    },
    /// [342. Power of Four](https://leetcode.com/problems/power-of-four/)
    IsPowerOfFour {
        n: i32,
    },
    /// [326. Power of Three](https://leetcode.com/problems/power-of-three/)
    IsPowerOfThree {
        n: i32,
    },
    /// [231. Power of Two](https://leetcode.com/problems/power-of-two/)
    IsPowerOfTwo {
        n: i32,
    },
    /// [392. Is Subsequence](https://leetcode.com/problems/is-subsequence/)
    IsSubsequence {
        s: String,
        t: String,
    },
    /// [45. Jump Game II](https://leetcode.com/problems/jump-game-ii/)
    Jump {
        nums: String,
    },
    /// [1431. Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)
    KidsWithCandies {
        candies: String,
        extra_candies: i32,
    },
    /// [779. K-th Symbol in Grammar](https://leetcode.com/problems/k-th-symbol-in-grammar/)
    KthGrammar {
        n: i32,
        k: i32,
    },
    /// [1732. Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/)
    LargestAltitude {
        gain: String,
    },
    /// [409. Longest Palindrome](https://leetcode.com/problems/longest-palindrome/)
    LongestPalindrome {
        s: String,
    },
    /// [169. Majority Element](https://leetcode.com/problems/majority-element/)
    MajorityElement {
        nums: String,
    },
    /// [121. Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/?envType=study-plan-v2&envId=top-interview-150)
    MaxProfit1 {
        prices: String,
    },
    /// [122. Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/?envType=study-plan-v2&envId=top-interview-150)
    MaxProfit2 {
        prices: String,
    },
    /// [1043. Partition Array for Maximum Sum](https://leetcode.com/problems/partition-array-for-maximum-sum/)
    MaxSumAfterPartitioning {
        arr: String,
        k: i32,
    },
    /// [1768. Merge Strings Alternately](https://leetcode.com/problems/merge-strings-alternately/)
    MergeAlternately {
        word1: String,
        word2: String,
    },
    // [21. Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)
    MergeTwoLists {
        list1: String,
        list2: String,
    },
    /// [88. Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/?envType=study-plan-v2&envId=top-interview-150)
    Merge {
        nums1: String,
        m: i32,
        nums2: String,
        n: i32,
    },
    // [876. Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/)
    MiddleNode {
        head: String,
    },
    /// [746. Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/)
    MinCostClimbingStairs {
        cost: String,
    },
    /// [875. Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/)
    MinEatingSpeed {
        piles: String,
        h: i32,
    },
    /// [1969. Minimum Non-Zero Product of the Array Elements](https://leetcode.com/problems/minimum-non-zero-product-of-the-array-elements/)
    MinNonZeroProduct {
        p: u64,
    },
    /// [2492. Minimum Score of a Path Between Two Cities](https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/)
    MinScore {
        n: i32,
        roads: String,
    },
    /// [283. Move Zeroes](https://leetcode.com/problems/move-zeroes/)
    MoveZeroes {
        nums: String,
    },
    /// [1079. Letter Tile Possibilities](https://leetcode.com/problems/letter-tile-possibilities/description/)
    NumTilePossibilities {
        tiles: String,
    },
    /// [46. Permutations](https://leetcode.com/problems/permutations/)
    Permute {
        nums: String,
    },
    /// [724. Find Pivot Index](https://leetcode.com/problems/find-pivot-index/)
    PivotIndex {
        nums: String,
    },
    /// [486. Predict the Winner](https://leetcode.com/problems/predict-the-winner/)
    PredictTheWinner {
        nums: String,
    },
    /// [238. Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)
    ProductExceptSelf {
        nums: String,
    },
    /// [401. Binary Watch](https://leetcode.com/problems/binary-watch/)
    ReadBinaryWatch {
        turned_on: i32,
    },
    /// [26. Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150)
    RemoveDuplicates1 {
        nums: String,
    },
    /// [80. Remove Duplicates from Sorted Array II](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/?envType=study-plan-v2&envId=top-interview-150)
    RemoveDuplicates2 {
        nums: String,
    },
    /// [27. Remove Element](https://leetcode.com/problems/remove-element/?envType=study-plan-v2&envId=top-interview-150)
    RemoveElement {
        nums: String,
        val: i32,
    },
    // [203. Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)
    RemoveElements {
        head: String,
        val: i32,
    },
    // [2487. Remove Nodes From Linked List](https://leetcode.com/problems/remove-nodes-from-linked-list/)
    RemoveNodes {
        head: String,
    },
    // [143. Reorder List](https://leetcode.com/problems/reorder-list/)
    ReorderList {
        head: String,
    },
    // [206. Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)
    ReverseList {
        head: String,
    },
    /// [345. Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string/)
    ReverseVowels {
        s: String,
    },
    /// [151. Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)
    ReverseWords {
        s: String,
    },
    /// [189. Rotate Array](https://leetcode.com/problems/rotate-array/?envType=study-plan-v2&envId=top-interview-150)
    Rotate {
        nums: String,
        k: i32,
    },
    /// [1480. Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)
    RunningSum {
        nums: String,
    },
    /// [704. Binary Search](https://leetcode.com/problems/binary-search/)
    Search {
        nums: String,
        target: i32,
    },
    /// [1011. Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/)
    ShipWithinDays {
        weights: String,
        days: i32,
    },
    /// [877. Stone Game](https://leetcode.com/problems/stone-game/)
    StoneGame {
        piles: String,
    },
    /// [1863. Sum of All Subset XOR Totals](https://leetcode.com/problems/sum-of-all-subset-xor-totals/)
    SubsetXorSum {
        nums: String,
    },
    /// [78. Subsets](https://leetcode.com/problems/subsets/)
    Subsets {
        nums: String,
    },
    // [24. Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)
    SwapPairs {
        head: String,
    },
    /// [904. Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets/)
    TotalFruit {
        fruits: String,
    },
    /// [1137. N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/)
    Tribonacci {
        n: i32,
    },
    /// [1884. Egg Drop With 2 Eggs and N Floors](https://leetcode.com/problems/egg-drop-with-2-eggs-and-n-floors/)
    TwoEggDrop {
        n: i32,
    },
    /// [2348. Number of Zero-Filled Subarrays](https://leetcode.com/problems/number-of-zero-filled-subarrays/)
    ZeroFilledSubarray {
        nums: String,
    },
    /// [55. Jump Game](https://leetcode.com/problems/jump-game/?envType=study-plan-v2&envId=top-interview-150)
    CanJump {
        nums: String,
    },
    /// [274. H-Index](https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&envId=top-interview-150)
    HIndex {
        citations: String,
    },
    /// [134. Gas Station](https://leetcode.com/problems/gas-station/?envType=study-plan-v2&envId=top-interview-150)
    CanCompleteCircuit {
        gas: String,
        cost: String,
    },
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
        Commands::Generate { num_rows } => {
            println!("{:?}", Solution::generate(*num_rows));
        }
        Commands::GetKth { lo, hi, k } => {
            println!("{:?}", Solution::get_kth(*lo, *hi, *k));
        }
        Commands::GetMaximumGenerated { n } => {
            println!("{:?}", Solution::get_maximum_generated(*n));
        }
        Commands::GetRow { row_index } => {
            println!("{:?}", Solution::get_row(*row_index));
        }
        Commands::IsAnagram { s, t } => {
            println!("{:?}", Solution::is_anagram(s.clone(), t.clone()));
        }
        Commands::IsIsomorphic { s, t } => {
            println!("{:?}", Solution::is_isomorphic(s.clone(), t.clone()));
        }
        Commands::IsPalindrome { head } => {
            let head = utils::parse_list_i32(head);
            let mut list = SinglyLinkedList::from(head);
            println!("{:?}", Solution::is_palindrome(list.head.take()));
        }
        Commands::IsPowerOfFour { n } => {
            println!("{:?}", Solution::is_power_of_four(*n));
        }
        Commands::IsPowerOfThree { n } => {
            println!("{:?}", Solution::is_power_of_three(*n));
        }
        Commands::IsPowerOfTwo { n } => {
            println!("{:?}", Solution::is_power_of_two(*n));
        }
        Commands::IsSubsequence { s, t } => {
            println!("{:?}", Solution::is_subsequence(s.clone(), t.clone()));
        }
        Commands::Jump { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::jump(nums));
        }
        Commands::KidsWithCandies {
            candies,
            extra_candies,
        } => {
            let candies = utils::parse_list_i32(candies);
            println!("{:?}", Solution::kids_with_candies(candies, *extra_candies));
        }
        Commands::KthGrammar { n, k } => {
            println!("{:?}", Solution::kth_grammar(*n, *k));
        }
        Commands::LargestAltitude { gain } => {
            let gain = utils::parse_list_i32(gain);
            println!("{:?}", Solution::largest_altitude(gain));
        }
        Commands::LongestPalindrome { s } => {
            println!("{:?}", Solution::longest_palindrome(s.clone()));
        }
        Commands::MajorityElement { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::majority_element(nums));
        }
        Commands::MaxProfit1 { prices } => {
            let prices = utils::parse_list_i32(prices);
            println!("{:?}", Solution::max_profit1(prices));
        }
        Commands::MaxProfit2 { prices } => {
            let prices = utils::parse_list_i32(prices);
            println!("{:?}", Solution::max_profit2(prices));
        }
        Commands::MaxSumAfterPartitioning { arr, k } => {
            let arr = utils::parse_list_i32(arr);
            println!("{:?}", Solution::max_sum_after_partitioning(arr, *k));
        }
        Commands::MergeAlternately { word1, word2 } => {
            println!(
                "{:?}",
                Solution::merge_alternately(word1.clone(), word2.clone())
            );
        }
        Commands::MergeTwoLists { list1, list2 } => {
            let mut list1 = SinglyLinkedList::from(utils::parse_list_i32(list1));
            let mut list2 = SinglyLinkedList::from(utils::parse_list_i32(list2));
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::merge_two_lists(list1.head.take(), list2.head.take()),
                }
            );
        }
        Commands::Merge { nums1, m, nums2, n } => {
            let mut nums1 = utils::parse_list_i32(nums1);
            let mut nums2 = utils::parse_list_i32(nums2);
            println!("{:?}", Solution::merge(&mut nums1, *m, &mut nums2, *n));
        }
        Commands::MiddleNode { head } => {
            let head = utils::parse_list_i32(head);
            let mut list = SinglyLinkedList::from(head);
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::middle_node(list.head.take())
                }
            );
        }
        Commands::MinCostClimbingStairs { cost } => {
            let cost = utils::parse_list_i32(cost);
            println!("{:?}", Solution::min_cost_climbing_stairs(cost));
        }
        Commands::MinEatingSpeed { piles, h } => {
            let piles = utils::parse_list_i32(piles);
            println!("{:?}", Solution::min_eating_speed(piles, *h));
        }
        Commands::MinNonZeroProduct { p } => {
            println!("{:?}", Solution::min_non_zero_product(*p));
        }
        Commands::MinScore { n, roads } => {
            let roads = utils::parse_2d_list_i32(roads);
            println!("{:?}", Solution::min_score(*n, roads));
        }
        Commands::MoveZeroes { nums } => {
            let mut nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::move_zeroes(&mut nums));
        }
        Commands::NumTilePossibilities { tiles } => {
            println!("{:?}", Solution::num_tile_possibilities(tiles.clone()));
        }
        Commands::Permute { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::permute(nums));
        }
        Commands::PivotIndex { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::pivot_index(nums));
        }
        Commands::PredictTheWinner { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::predict_the_winner(nums));
        }
        Commands::ProductExceptSelf { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::product_except_self(nums));
        }
        Commands::ReadBinaryWatch { turned_on } => {
            println!("{:?}", Solution::read_binary_watch(*turned_on));
        }
        Commands::RemoveDuplicates1 { nums } => {
            let mut nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::remove_duplicates1(&mut nums));
        }
        Commands::RemoveDuplicates2 { nums } => {
            let mut nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::remove_duplicates2(&mut nums));
        }
        Commands::RemoveElement { nums, val } => {
            let mut nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::remove_element(&mut nums, *val));
        }
        Commands::RemoveElements { head, val } => {
            let head = utils::parse_list_i32(head);
            let mut list = SinglyLinkedList::from(head);
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::remove_elements(list.head.take(), *val)
                }
            );
        }
        Commands::RemoveNodes { head } => {
            let head = utils::parse_list_i32(head);
            let mut list = SinglyLinkedList::from(head);
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::remove_nodes(list.head.take())
                }
            );
        }
        Commands::ReorderList { head } => {
            let head = utils::parse_list_i32(head);
            let mut list = SinglyLinkedList::from(head);
            Solution::reorder_list(&mut list.head);
            println!("{:?}", list);
        }
        Commands::ReverseList { head } => {
            let head = utils::parse_list_i32(head);
            let mut list = SinglyLinkedList::from(head);
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::reverse_list(list.head.take())
                }
            );
        }
        Commands::ReverseVowels { s } => {
            println!("{:?}", Solution::reverse_vowels(s.clone()));
        }
        Commands::ReverseWords { s } => {
            println!("{:?}", Solution::reverse_words(s.clone()));
        }
        Commands::Rotate { nums, k } => {
            let mut nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::rotate(&mut nums, *k));
        }
        Commands::RunningSum { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::running_sum(nums));
        }
        Commands::Search { nums, target } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::search(nums, *target));
        }
        Commands::ShipWithinDays { weights, days } => {
            let weights = utils::parse_list_i32(weights);
            println!("{:?}", Solution::ship_within_days(weights, *days));
        }
        Commands::StoneGame { piles } => {
            let piles = utils::parse_list_i32(piles);
            println!("{:?}", Solution::stone_game(piles));
        }
        Commands::SubsetXorSum { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::subset_xor_sum(nums));
        }
        Commands::Subsets { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::subsets(nums));
        }
        Commands::SwapPairs { head } => {
            let head = utils::parse_list_i32(head);
            let mut list = SinglyLinkedList::from(head);
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::swap_pairs(list.head.take())
                }
            );
        }
        Commands::TotalFruit { fruits } => {
            let fruits = utils::parse_list_i32(fruits);
            println!("{:?}", Solution::total_fruit(fruits));
        }
        Commands::Tribonacci { n } => {
            println!("{:?}", Solution::tribonacci(*n));
        }
        Commands::TwoEggDrop { n } => {
            println!("{:?}", Solution::two_egg_drop(*n));
        }
        Commands::ZeroFilledSubarray { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::zero_filled_subarray(nums));
        }
        Commands::CanJump { nums } => {
            let nums = utils::parse_list_i32(nums);
            println!("{:?}", Solution::can_jump(nums));
        }
        Commands::HIndex { citations } => {
            let citations = utils::parse_list_i32(citations);
            println!("{:?}", Solution::h_index(citations));
        }
        Commands::CanCompleteCircuit { gas, cost } => {
            let gas = utils::parse_list_i32(gas);
            let cost = utils::parse_list_i32(cost);
            println!("{:?}", Solution::can_complete_circuit(gas, cost));
        }
    }
}
