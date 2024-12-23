mod utils;

use crate::utils::*;
use clap::{Parser, Subcommand};
use practice_leetcode_rs::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, propagate_version = true)]
#[command(after_help = "See 'solution help <command>' for more information on a specific command.")]
struct Cli {
    #[arg(short, long, value_name = "PATH", long_help = FILE_HELP_MSG)]
    file: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// [797. All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/)
    AllPathsSourceTarget {},
    /// [894. All Possible Full Binary Trees](https://leetcode.com/problems/all-possible-full-binary-trees/)
    AllPossibleFbt {},
    /// [257. Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)
    BinaryTreePaths {},
    /// [605. Can Place Flowers](https://leetcode.com/problems/can-place-flowers/)
    CanPlaceFlowers {},
    /// [217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)
    ContainsDuplicate {},
    /// [338. Counting Bits](https://leetcode.com/problems/counting-bits/)
    CountBits {},
    /// [2044. Count Number of Maximum Bitwise-OR Subsets](https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/)
    CountMaxOrSubsets {},
    /// [1523. Count Odd Numbers in an Interval Range](https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/)
    CountOdds {},
    /// [2316. Count Unreachable Pairs of Nodes in an Undirected Graph](https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/)
    CountPairs {},
    /// [1277. Count Square Submatrices with All Ones](https://leetcode.com/problems/count-square-submatrices-with-all-ones/)
    CountSquares {},
    /// [1638. Count Substrings That Differ by One Character](https://leetcode.com/problems/count-substrings-that-differ-by-one-character/)
    CountSubstrings {},
    /// [1641. Count Sorted Vowel Strings](https://leetcode.com/problems/count-sorted-vowel-strings/)
    CountVowelStrings {},
    /// [394. Decode String](https://leetcode.com/problems/decode-string/)
    DecodeString {},
    /// [241. Different Ways to Add Parentheses](https://leetcode.com/problems/different-ways-to-add-parentheses/)
    DiffWaysToCompute {},
    /// [2305. Fair Distribution of Cookies](https://leetcode.com/problems/fair-distribution-of-cookies/)
    DistributeCookies {},
    /// [1025. Divisor Game](https://leetcode.com/problems/divisor-game/description/)
    DivisorGame {},
    /// [509. Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)
    #[command(help_template = FIB_HELP_TEMPLATE)]
    Fib {},
    /// [1706. Where Will the Ball Fall](https://leetcode.com/problems/where-will-the-ball-fall/description/)
    FindBall {},
    /// [1706. Where Will the Ball Fall](https://leetcode.com/problems/where-will-the-ball-fall/description/)
    FindDifference {},
    /// [1545. Find Kth Bit in Nth Binary String](https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/)
    FindKthBit {},
    /// [1539. Kth Missing Positive Number](https://leetcode.com/problems/kth-missing-positive-number/)
    FindKthPositive {},
    /// [643. Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/)
    FindMaxAverage {},
    /// [1823. Find the Winner of the Circular Game](https://leetcode.com/problems/find-the-winner-of-the-circular-game/)
    FindTheWinner {},
    /// [1071. Greatest Common Divisor of Strings](https://leetcode.com/problems/greatest-common-divisor-of-strings/)
    GcdOfStrings {},
    /// [22. Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)
    GenerateParenthesis {},
    /// [118. Pascal's Triangle](https://leetcode.com/problems/pascals-triangle/)
    Generate {},
    /// [1387. Sort Integers by The Power Value](https://leetcode.com/problems/sort-integers-by-the-power-value/)
    GetKth {},
    /// [1646. Get Maximum in Generated Array](https://leetcode.com/problems/get-maximum-in-generated-array/description/)
    GetMaximumGenerated {},
    /// [119. Pascal's Triangle II](https://leetcode.com/problems/pascals-triangle-ii/)
    GetRow {},
    /// [112. Path Sum](https://leetcode.com/problems/path-sum/)
    HasPathSum {},
    /// [242. Valid Anagram](https://leetcode.com/problems/valid-anagram/)
    IsAnagram {},
    /// [205. Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/)
    IsIsomorphic {},
    /// [234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/description/)
    IsPalindrome {},
    /// [342. Power of Four](https://leetcode.com/problems/power-of-four/)
    IsPowerOfFour {},
    /// [326. Power of Three](https://leetcode.com/problems/power-of-three/)
    IsPowerOfThree {},
    /// [231. Power of Two](https://leetcode.com/problems/power-of-two/)
    IsPowerOfTwo {},
    /// [392. Is Subsequence](https://leetcode.com/problems/is-subsequence/)
    IsSubsequence {},
    /// [45. Jump Game II](https://leetcode.com/problems/jump-game-ii/)
    Jump {},
    /// [1431. Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)
    KidsWithCandies {},
    /// [779. K-th Symbol in Grammar](https://leetcode.com/problems/k-th-symbol-in-grammar/)
    KthGrammar {},
    /// [1732. Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/)
    LargestAltitude {},
    /// [409. Longest Palindrome](https://leetcode.com/problems/longest-palindrome/)
    LongestPalindrome {},
    /// [169. Majority Element](https://leetcode.com/problems/majority-element/)
    MajorityElement {},
    /// [121. Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/?envType=study-plan-v2&envId=top-interview-150)
    MaxProfit1 {},
    /// [122. Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/?envType=study-plan-v2&envId=top-interview-150)
    MaxProfit2 {},
    /// [1043. Partition Array for Maximum Sum](https://leetcode.com/problems/partition-array-for-maximum-sum/)
    MaxSumAfterPartitioning {},
    /// [1768. Merge Strings Alternately](https://leetcode.com/problems/merge-strings-alternately/)
    MergeAlternately {},
    /// [21. Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)
    MergeTwoLists {},
    /// [88. Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/?envType=study-plan-v2&envId=top-interview-150)
    Merge {},
    /// [876. Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/)
    MiddleNode {},
    /// [746. Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/)
    MinCostClimbingStairs {},
    /// [875. Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/)
    MinEatingSpeed {},
    /// [1969. Minimum Non-Zero Product of the Array Elements](https://leetcode.com/problems/minimum-non-zero-product-of-the-array-elements/)
    MinNonZeroProduct {},
    /// [2492. Minimum Score of a Path Between Two Cities](https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/)
    MinScore {},
    /// [283. Move Zeroes](https://leetcode.com/problems/move-zeroes/)
    MoveZeroes {},
    /// [1079. Letter Tile Possibilities](https://leetcode.com/problems/letter-tile-possibilities/description/)
    NumTilePossibilities {},
    /// [46. Permutations](https://leetcode.com/problems/permutations/)
    Permute {},
    /// [724. Find Pivot Index](https://leetcode.com/problems/find-pivot-index/)
    PivotIndex {},
    /// [486. Predict the Winner](https://leetcode.com/problems/predict-the-winner/)
    PredictTheWinner {},
    /// [238. Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)
    ProductExceptSelf {},
    /// [401. Binary Watch](https://leetcode.com/problems/binary-watch/)
    ReadBinaryWatch {},
    /// [26. Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150)
    RemoveDuplicates1 {},
    /// [80. Remove Duplicates from Sorted Array II](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/?envType=study-plan-v2&envId=top-interview-150)
    RemoveDuplicates2 {},
    /// [27. Remove Element](https://leetcode.com/problems/remove-element/?envType=study-plan-v2&envId=top-interview-150)
    RemoveElement {},
    /// [203. Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)
    RemoveElements {},
    /// [2487. Remove Nodes From Linked List](https://leetcode.com/problems/remove-nodes-from-linked-list/)
    RemoveNodes {},
    /// [143. Reorder List](https://leetcode.com/problems/reorder-list/)
    ReorderList {},
    /// [206. Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)
    ReverseList {},
    /// [345. Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string/)
    ReverseVowels {},
    /// [151. Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)
    ReverseWords {},
    /// [189. Rotate Array](https://leetcode.com/problems/rotate-array/?envType=study-plan-v2&envId=top-interview-150)
    Rotate {},
    /// [1480. Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)
    RunningSum {},
    /// [704. Binary Search](https://leetcode.com/problems/binary-search/)
    Search {},
    /// [1011. Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/)
    ShipWithinDays {},
    /// [877. Stone Game](https://leetcode.com/problems/stone-game/)
    StoneGame {},
    /// [1863. Sum of All Subset XOR Totals](https://leetcode.com/problems/sum-of-all-subset-xor-totals/)
    SubsetXorSum {},
    /// [78. Subsets](https://leetcode.com/problems/subsets/)
    Subsets {},
    /// [24. Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)
    SwapPairs {},
    /// [904. Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets/)
    TotalFruit {},
    /// [1137. N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/)
    Tribonacci {},
    /// [1884. Egg Drop With 2 Eggs and N Floors](https://leetcode.com/problems/egg-drop-with-2-eggs-and-n-floors/)
    TwoEggDrop {},
    /// [2348. Number of Zero-Filled Subarrays](https://leetcode.com/problems/number-of-zero-filled-subarrays/)
    ZeroFilledSubarray {},
    /// [55. Jump Game](https://leetcode.com/problems/jump-game/?envType=study-plan-v2&envId=top-interview-150)
    CanJump {},
    /// [274. H-Index](https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&envId=top-interview-150)
    HIndex {},
    /// [134. Gas Station](https://leetcode.com/problems/gas-station/?envType=study-plan-v2&envId=top-interview-150)
    CanCompleteCircuit {},
    /// [58. Length of Last Word](https://leetcode.com/problems/length-of-last-word/?envType=study-plan-v2&envId=top-interview-150)
    LengthOfLastWord {},
    /// [13. Roman to Integer](https://leetcode.com/problems/roman-to-integer/description/)
    RomanToInt {},
    /// [14. Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/description/)
    LongestCommonPrefix {},
    /// [28. Find the Index of the First Occurrence in a String](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/?envType=study-plan-v2&envId=top-interview-150)
    StrStr {},
    /// [383. Ransom Note](https://leetcode.com/problems/ransom-note/description/?envType=study-plan-v2&envId=top-interview-150)
    CanConstruct {},
    /// [290. Word Pattern](https://leetcode.com/problems/word-pattern/description/?envType=study-plan-v2&envId=top-interview-150)
    WordPattern {},
    /// [1. Two Sum](https://leetcode.com/problems/two-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    TwoSum1 {},
    /// [167. Two Sum II - Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/?envType=study-plan-v2&envId=top-interview-150)
    TwoSum2 {},
    /// [202. Happy Number](https://leetcode.com/problems/happy-number/description/?envType=study-plan-v2&envId=top-interview-150)
    IsHappy {},
    /// [219. Contains Duplicate II](https://leetcode.com/problems/contains-duplicate-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ContainsNearbyDuplicate {},
    /// [49. Group Anagrams](https://leetcode.com/problems/group-anagrams/description/?envType=study-plan-v2&envId=top-interview-150)
    GroupAnagrams {},
    /// [128. Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/description/?envType=study-plan-v2&envId=top-interview-150)
    LongestConsecutive {},
    /// [12. Integer to Roman](https://leetcode.com/problems/integer-to-roman/description/?envType=study-plan-v2&envId=top-interview-150)
    IntToRoman {},
    /// [6. Zigzag Conversion](https://leetcode.com/problems/zigzag-conversion/description/)
    Convert {},
    /// [135. Candy](https://leetcode.com/problems/candy/description/?envType=study-plan-v2&envId=top-interview-150)
    Candy {},
    /// [17. Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/?envType=study-plan-v2&envId=top-interview-150)
    LetterCombinations {},
    /// [77. Combinations](https://leetcode.com/problems/combinations/description/?envType=study-plan-v2&envId=top-interview-150)
    Combine {},
    /// [39. Combination Sum](https://leetcode.com/problems/combination-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    CombinationSum {},
    /// [52. N-Queens II](https://leetcode.com/problems/n-queens-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    TotalNQueens {},
    /// [79. Word Search](https://leetcode.com/problems/word-search/description/?envType=study-plan-v2&envId=top-interview-150)
    Exist {},
    /// [108. Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    SortedArrayToBst {},
    /// [234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/description/)
    IsPalindromeStr {},
    /// # [2. Add Two Numbers](https://leetcode.com/problems/add-two-numbers/description/?envType=study-plan-v2&envId=top-interview-150)
    AddTwoNumbers {},
    /// [11. Container With Most Water](https://leetcode.com/problems/container-with-most-water/description/?envType=study-plan-v2&envId=top-interview-150)
    MaxArea {},
    /// [15. 3Sum](https://leetcode.com/problems/3sum/description/?envType=study-plan-v2&envId=top-interview-150)
    ThreeSum {},
    /// [42. Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/description/?envType=study-plan-v2&envId=top-interview-150)
    Trap {},
    /// [68. Text Justification](https://leetcode.com/problems/text-justification/description/?envType=study-plan-v2&envId=top-interview-150)
    FullJustify {},
    /// [209. Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    MinSubArrayLen {},
    /// [3. Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/description/?envType=study-plan-v2&envId=top-interview-150)
    LengthOfLongestSubstring {},
    /// [36. Valid Sudoku](https://leetcode.com/problems/valid-sudoku/description/?envType=study-plan-v2&envId=top-interview-150)
    IsValidSudoku {},
    /// [54. Spiral Matrix](https://leetcode.com/problems/spiral-matrix/description/?envType=study-plan-v2&envId=top-interview-150)
    SpiralOrder {},
    /// [48. Rotate Image](https://leetcode.com/problems/rotate-image/description/?envType=study-plan-v2&envId=top-interview-150)
    RotateImage {},
    /// [73. Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes/description/?envType=study-plan-v2&envId=top-interview-150)
    SetZeroes {},
    /// [289. Game of Life](https://leetcode.com/problems/game-of-life/description/?envType=study-plan-v2&envId=top-interview-150)
    GameOfLife {},
    /// [228. Summary Ranges](https://leetcode.com/problems/summary-ranges/description/?envType=study-plan-v2&envId=top-interview-150)
    SummaryRanges {},
    /// [56. Merge Intervals](https://leetcode.com/problems/merge-intervals/description/?envType=study-plan-v2&envId=top-interview-150)
    MergeIntervals {},
    /// [30. Substring with Concatenation of All Words](https://leetcode.com/problems/substring-with-concatenation-of-all-words/description/?envType=study-plan-v2&envId=top-interview-150)
    FindSubstring {},
    /// [57. Insert Interval](https://leetcode.com/problems/insert-interval/description/?envType=study-plan-v2&envId=top-interview-150)
    Insert {},
    /// [452. Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/description/?envType=study-plan-v2&envId=top-interview-150)
    FindMinArrowShots {},
    /// [76. Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/description/?envType=study-plan-v2&envId=top-interview-150)
    MinWindow {},
    /// [71. Simplify Path](https://leetcode.com/problems/simplify-path/description/?envType=study-plan-v2&envId=top-interview-150)
    SimplifyPath {},
    /// [92. Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ReverseBetween {},
    /// [155. Min Stack](https://leetcode.com/problems/min-stack/description/?envType=study-plan-v2&envId=top-interview-150)
    MinStack {},
    /// [150. Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/description/?envType=study-plan-v2&envId=top-interview-150)
    EvalRpn {},
    /// [224. Basic Calculator](https://leetcode.com/problems/basic-calculator/description/?envType=study-plan-v2&envId=top-interview-150)
    Calculate {},
    /// [19. Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/?envType=study-plan-v2&envId=top-interview-150)
    RemoveNthFromEnd {},
    /// [82. Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    DeleteDuplicates {},
    /// [61. Rotate List](https://leetcode.com/problems/rotate-list/description/?envType=study-plan-v2&envId=top-interview-150)
    RotateRight {},
    /// [86. Partition List](https://leetcode.com/problems/partition-list/description/?envType=study-plan-v2&envId=top-interview-150)
    Partition {},
    /// [104. Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    MaxDepth {},
    /// [100. Same Tree](https://leetcode.com/problems/same-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    IsSameTree {},
    /// [226. Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    InvertTree {},
    ///[101. Symmetric Tree](https://leetcode.com/problems/symmetric-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    IsSymmetric {},
    /// [105. Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/?envType=study-plan-v2&envId=top-interview-150)
    BuildTreeFromPreorderAndInorder {},
    /// [106. Construct Binary Tree from Inorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/?envType=study-plan-v2&envId=top-interview-150)
    BuildTreeFromInorderAndPostorder {},
    /// [114. Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/description/?envType=study-plan-v2&envId=top-interview-150)
    Flatten {},
    /// [222. Count Complete Tree Nodes](https://leetcode.com/problems/count-complete-tree-nodes/description/?envType=study-plan-v2&envId=top-interview-150)
    CountNodes {},
    /// [129. Sum Root to Leaf Numbers](https://leetcode.com/problems/sum-root-to-leaf-numbers/description/?envType=study-plan-v2&envId=top-interview-150)
    SumNumbers {},
    /// [124. Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    MaxPathSum {},
    /// [236. Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    LowestCommonAncestor {},
    /// [199. Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/description/?envType=study-plan-v2&envId=top-interview-150)
    RightSideView {},
    /// [102. Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/description/?envType=study-plan-v2&envId=top-interview-150)
    LevelOrder {},
    /// [637. Average of Levels in Binary Tree](https://leetcode.com/problems/average-of-levels-in-binary-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    AverageOfLevels {},
    /// [103. Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/description/?envType=study-plan-v2&envId=top-interview-150)
    ZigzagLevelOrder {},
    /// [530. Minimum Absolute Difference in BST](https://leetcode.com/problems/minimum-absolute-difference-in-bst/description/?envType=study-plan-v2&envId=top-interview-150)
    GetMinimumDifference {},
    /// [230. Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/description/?envType=study-plan-v2&envId=top-interview-150)
    KthSmallest {},
    /// [98. Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    IsValidBst {},
    /// [200. Number of Islands](https://leetcode.com/problems/number-of-islands/description/?envType=study-plan-v2&envId=top-interview-150)
    NumIslands {},
    /// [130. Surrounded Regions](https://leetcode.com/problems/surrounded-regions/description/?envType=study-plan-v2&envId=top-interview-150)
    Solve {},
    /// [399. Evaluate Division](https://leetcode.com/problems/evaluate-division/description/?envType=study-plan-v2&envId=top-interview-150)
    CalcEquation {},
    /// [207. Course Schedule](https://leetcode.com/problems/course-schedule/description/?envType=study-plan-v2&envId=top-interview-150)
    CanFinish {},
    /// [210. Course Schedule II](https://leetcode.com/problems/course-schedule-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    FindOrder {},
    /// [909. Snakes and Ladders](https://leetcode.com/problems/snakes-and-ladders/description/?envType=study-plan-v2&envId=top-interview-150)
    SnakesAndLadders {},
    /// [433. Minimum Genetic Mutation](https://leetcode.com/problems/minimum-genetic-mutation/description/?envType=study-plan-v2&envId=top-interview-150)
    MinMutation {},
    /// [127. Word Ladder](https://leetcode.com/problems/word-ladder/description/?envType=study-plan-v2&envId=top-interview-150)
    LadderLength {},
    /// [148. Sort List](https://leetcode.com/problems/sort-list/description/?envType=study-plan-v2&envId=top-interview-150)
    SortList {},
    /// [212. Word Search II](https://leetcode.com/problems/word-search-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    FindWords {},
    /// [53. Maximum Subarray](https://leetcode.com/problems/maximum-subarray/description/?envType=study-plan-v2&envId=top-interview-150)
    MaxSubArray {},
    /// [918. Maximum Sum Circular Subarray](https://leetcode.com/problems/maximum-sum-circular-subarray/description/?envType=study-plan-v2&envId=top-interview-150)
    MaxSubarraySumCircular {},
    /// [74. Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/description/?envType=study-plan-v2&envId=top-interview-150)
    SearchMatrix {},
    /// [35. Search Insert Position](https://leetcode.com/problems/search-insert-position/description/?envType=study-plan-v2&envId=top-interview-150)
    SearchInsert {},
    /// [162. Find Peak Element](https://leetcode.com/problems/find-peak-element/description/?envType=study-plan-v2&envId=top-interview-150)
    FindPeakElement {},
    /// [33. Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150)
    SearchRotatedSortedArray {},
    /// [34. Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150)
    SearchRange {},
    /// [153. Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150)
    FindMin {},
    /// [4. Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/description/?envType=study-plan-v2&envId=top-interview-150)
    FindMedianSortedArrays {},
    /// [23. Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/description/?envType=study-plan-v2&envId=top-interview-150)
    MergeKLists {},
    /// [67. Add Binary](https://leetcode.com/problems/add-binary/description/?envType=study-plan-v2&envId=top-interview-150)
    AddBinary {},
    /// [190. Reverse Bits](https://leetcode.com/problems/reverse-bits/description/?envType=study-plan-v2&envId=top-interview-150)
    ReverseBits {},
    /// [191. Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/description/?envType=study-plan-v2&envId=top-interview-150)
    HammingWeight {},
    /// [136. Single Number](https://leetcode.com/problems/single-number/description/?envType=study-plan-v2&envId=top-interview-150)
    SingleNumber {},
    /// [137. Single Number II](https://leetcode.com/problems/single-number-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    SingleNumber2 {},
}

fn main() {
    let cli = Cli::parse();

    // read from file or stdin
    let input = if let Some(path) = cli.file.as_deref() {
        read_from_file(path).unwrap()
    } else {
        read_from_stdin().unwrap()
    };

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::AllPathsSourceTarget {} => {
            let graph = parse_2d_i32_list(&input);
            println!("{:?}", Solution::all_paths_source_target(graph));
        }
        Commands::AllPossibleFbt {} => {
            let n = parse_i32(&input);
            let roots = Solution::all_possible_fbt(n);
            let trees = roots
                .into_iter()
                .map(|root| Tree { root })
                .collect::<Vec<Tree>>();
            println!("{:?}", trees);
        }
        Commands::BinaryTreePaths {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            let root = tree.root.clone();
            println!("{:?}", Solution::binary_tree_paths(root));
        }
        Commands::CanPlaceFlowers {} => {
            let (flowerbed, n) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::can_place_flowers(flowerbed, n));
        }
        Commands::ContainsDuplicate {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::contains_duplicate(nums));
        }
        Commands::CountBits {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::count_bits(n));
        }
        Commands::CountMaxOrSubsets {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::count_max_or_subsets(nums));
        }
        Commands::CountOdds {} => {
            let (low, high) = parse_two_i32(&input);
            println!("{:?}", Solution::count_odds(low, high));
        }
        Commands::CountPairs {} => {
            let (n, edges) = parse_i32_and_2d_i32_list(&input);
            println!("{:?}", Solution::count_pairs(n, edges));
        }
        Commands::CountSquares {} => {
            let matrix = parse_2d_i32_list(&input);
            println!("{:?}", Solution::count_squares(matrix));
        }
        Commands::CountSubstrings {} => {
            let (s, t) = parse_two_str(&input);
            println!("{:?}", Solution::count_substrings(s, t));
        }
        Commands::CountVowelStrings {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::count_vowel_strings(n));
        }
        Commands::DecodeString {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::decode_string(s));
        }
        Commands::DiffWaysToCompute {} => {
            let expression = parse_str(&input);
            println!("{:?}", Solution::diff_ways_to_compute(expression));
        }
        Commands::DistributeCookies {} => {
            let (cookies, k) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::distribute_cookies(cookies, k));
        }
        Commands::DivisorGame {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::divisor_game(n));
        }
        Commands::Fib {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::fib(n));
        }
        Commands::FindBall {} => {
            let grid = parse_2d_i32_list(&input);
            println!("{:?}", Solution::find_ball(grid));
        }
        Commands::FindDifference {} => {
            let (nums1, nums2) = parse_two_i32_list(&input);
            println!("{:?}", Solution::find_difference(nums1, nums2));
        }
        Commands::FindKthBit {} => {
            let (n, k) = parse_two_i32(&input);
            println!("{:?}", Solution::find_kth_bit(n, k));
        }
        Commands::FindKthPositive {} => {
            let (arr, k) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::find_kth_positive(arr, k));
        }
        Commands::FindMaxAverage {} => {
            let (nums, k) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::find_max_average(nums, k));
        }
        Commands::FindTheWinner {} => {
            let (n, k) = parse_two_i32(&input);
            println!("{:?}", Solution::find_the_winner(n, k));
        }
        Commands::GcdOfStrings {} => {
            let (str1, str2) = parse_two_str(&input);
            println!("{:?}", Solution::gcd_of_strings(str1, str2));
        }
        Commands::GenerateParenthesis {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::generate_parenthesis(n));
        }
        Commands::Generate {} => {
            let num_rows = parse_i32(&input);
            println!("{:?}", Solution::generate(num_rows));
        }
        Commands::GetKth {} => {
            let (lo, hi, k) = parse_three_i32(&input);
            println!("{:?}", Solution::get_kth(lo, hi, k));
        }
        Commands::GetMaximumGenerated {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::get_maximum_generated(n));
        }
        Commands::GetRow {} => {
            let row_index = parse_i32(&input);
            println!("{:?}", Solution::get_row(row_index));
        }
        Commands::HasPathSum {} => {
            let (nodes, sum) = parse_opt_i32_list_and_i32(&input);
            let tree = Tree::from(nodes);
            let root = tree.root.clone();
            println!("{:?}", Solution::has_path_sum(root, sum));
        }
        Commands::IsAnagram {} => {
            let (s, t) = parse_two_str(&input);
            println!("{:?}", Solution::is_anagram(s, t));
        }
        Commands::IsIsomorphic {} => {
            let (s, t) = parse_two_str(&input);
            println!("{:?}", Solution::is_isomorphic(s, t));
        }
        Commands::IsPalindrome {} => {
            let list = parse_i32_list(&input);
            let head = SinglyLinkedList::from(list).head;
            println!("{:?}", Solution::is_palindrome(head));
        }
        Commands::IsPowerOfFour {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::is_power_of_four(n));
        }
        Commands::IsPowerOfThree {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::is_power_of_three(n));
        }
        Commands::IsPowerOfTwo {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::is_power_of_two(n));
        }
        Commands::IsSubsequence {} => {
            let (s, t) = parse_two_str(&input);
            println!("{:?}", Solution::is_subsequence(s, t));
        }
        Commands::Jump {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::jump(nums));
        }
        Commands::KidsWithCandies {} => {
            let (candies, extra_candies) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::kids_with_candies(candies, extra_candies));
        }
        Commands::KthGrammar {} => {
            let (n, k) = parse_two_i32(&input);
            println!("{:?}", Solution::kth_grammar(n, k));
        }
        Commands::LargestAltitude {} => {
            let gain = parse_i32_list(&input);
            println!("{:?}", Solution::largest_altitude(gain));
        }
        Commands::LongestPalindrome {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::longest_palindrome(s));
        }
        Commands::MajorityElement {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::majority_element(nums));
        }
        Commands::MaxProfit1 {} => {
            let prices = parse_i32_list(&input);
            println!("{:?}", Solution::max_profit1(prices));
        }
        Commands::MaxProfit2 {} => {
            let prices = parse_i32_list(&input);
            println!("{:?}", Solution::max_profit2(prices));
        }
        Commands::MaxSumAfterPartitioning {} => {
            let (arr, k) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::max_sum_after_partitioning(arr, k));
        }
        Commands::MergeAlternately {} => {
            let (word1, word2) = parse_two_str(&input);
            println!("{:?}", Solution::merge_alternately(word1, word2));
        }
        Commands::MergeTwoLists {} => {
            let (list1, list2) = parse_two_i32_list(&input);
            let h1 = SinglyLinkedList::from(list1).head;
            let h2 = SinglyLinkedList::from(list2).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::merge_two_lists(h1, h2),
                }
            );
        }
        Commands::Merge {} => {
            let (mut nums1, m, mut nums2, n) = parse_two_i32_list_and_two_i32(&input);
            Solution::merge(&mut nums1, m, &mut nums2, n);
            println!("{:?}", nums1);
        }
        Commands::MiddleNode {} => {
            let list = parse_i32_list(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::middle_node(head)
                }
            );
        }
        Commands::MinCostClimbingStairs {} => {
            let cost = parse_i32_list(&input);
            println!("{:?}", Solution::min_cost_climbing_stairs(cost));
        }
        Commands::MinEatingSpeed {} => {
            let (piles, h) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::min_eating_speed(piles, h));
        }
        Commands::MinNonZeroProduct {} => {
            let p = parse_u64(&input);
            println!("{:?}", Solution::min_non_zero_product(p));
        }
        Commands::MinScore {} => {
            let (n, roads) = parse_i32_and_2d_i32_list(&input);
            println!("{:?}", Solution::min_score(n, roads));
        }
        Commands::MoveZeroes {} => {
            let mut nums = parse_i32_list(&input);
            println!("{:?}", Solution::move_zeroes(&mut nums));
        }
        Commands::NumTilePossibilities {} => {
            let tiles = parse_str(&input);
            println!("{:?}", Solution::num_tile_possibilities(tiles));
        }
        Commands::Permute {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::permute(nums));
        }
        Commands::PivotIndex {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::pivot_index(nums));
        }
        Commands::PredictTheWinner {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::predict_the_winner(nums));
        }
        Commands::ProductExceptSelf {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::product_except_self(nums));
        }
        Commands::ReadBinaryWatch {} => {
            let turned_on = parse_i32(&input);
            println!("{:?}", Solution::read_binary_watch(turned_on));
        }
        Commands::RemoveDuplicates1 {} => {
            let mut nums = parse_i32_list(&input);
            println!("{:?}", Solution::remove_duplicates1(&mut nums));
        }
        Commands::RemoveDuplicates2 {} => {
            let mut nums = parse_i32_list(&input);
            println!("{:?}", Solution::remove_duplicates2(&mut nums));
        }
        Commands::RemoveElement {} => {
            let (mut nums, val) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::remove_element(&mut nums, val));
        }
        Commands::RemoveElements {} => {
            let (list, val) = parse_i32_list_and_i32(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::remove_elements(head, val)
                }
            );
        }
        Commands::RemoveNodes {} => {
            let list = parse_i32_list(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::remove_nodes(head)
                }
            );
        }
        Commands::ReorderList {} => {
            let list = parse_i32_list(&input);
            let mut head = SinglyLinkedList::from(list).head;
            Solution::reorder_list(&mut head);
            println!("{:?}", SinglyLinkedList { head });
        }
        Commands::ReverseList {} => {
            let list = parse_i32_list(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::reverse_list(head)
                }
            );
        }
        Commands::ReverseVowels {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::reverse_vowels(s));
        }
        Commands::ReverseWords {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::reverse_words(s));
        }
        Commands::Rotate {} => {
            let (mut nums, k) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::rotate(&mut nums, k));
        }
        Commands::RunningSum {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::running_sum(nums));
        }
        Commands::Search {} => {
            let (nums, target) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::search(nums, target));
        }
        Commands::ShipWithinDays {} => {
            let (weights, days) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::ship_within_days(weights, days));
        }
        Commands::StoneGame {} => {
            let piles = parse_i32_list(&input);
            println!("{:?}", Solution::stone_game(piles));
        }
        Commands::SubsetXorSum {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::subset_xor_sum(nums));
        }
        Commands::Subsets {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::subsets(nums));
        }
        Commands::SwapPairs {} => {
            let list = parse_i32_list(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::swap_pairs(head)
                }
            );
        }
        Commands::TotalFruit {} => {
            let fruits = parse_i32_list(&input);
            println!("{:?}", Solution::total_fruit(fruits));
        }
        Commands::Tribonacci {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::tribonacci(n));
        }
        Commands::TwoEggDrop {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::two_egg_drop(n));
        }
        Commands::ZeroFilledSubarray {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::zero_filled_subarray(nums));
        }
        Commands::CanJump {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::can_jump(nums));
        }
        Commands::HIndex {} => {
            let citations = parse_i32_list(&input);
            println!("{:?}", Solution::h_index(citations));
        }
        Commands::CanCompleteCircuit {} => {
            let (gas, cost) = parse_two_i32_list(&input);
            println!("{:?}", Solution::can_complete_circuit(gas, cost));
        }
        Commands::LengthOfLastWord {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::length_of_last_word(s));
        }
        Commands::RomanToInt {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::roman_to_int(s));
        }
        Commands::LongestCommonPrefix {} => {
            let strs = parse_str_list(&input);
            println!("{:?}", Solution::longest_common_prefix(strs));
        }
        Commands::StrStr {} => {
            let (haystack, needle) = parse_two_str(&input);
            println!("{:?}", Solution::str_str(haystack, needle));
        }
        Commands::CanConstruct {} => {
            let (ransom_note, magazine) = parse_two_str(&input);
            println!("{:?}", Solution::can_construct(ransom_note, magazine));
        }
        Commands::WordPattern {} => {
            let (pattern, s) = parse_two_str(&input);
            println!("{:?}", Solution::word_pattern(pattern, s));
        }
        Commands::TwoSum1 {} => {
            let (nums, target) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::two_sum_1(nums, target));
        }
        Commands::TwoSum2 {} => {
            let (nums, target) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::two_sum_2(nums, target));
        }
        Commands::IsHappy {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::is_happy(n));
        }
        Commands::ContainsNearbyDuplicate {} => {
            let (nums, k) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::contains_nearby_duplicate(nums, k));
        }
        Commands::GroupAnagrams {} => {
            let strs = parse_str_list(&input);
            println!("{:?}", Solution::group_anagrams(strs));
        }
        Commands::LongestConsecutive {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::longest_consecutive(nums));
        }
        Commands::IntToRoman {} => {
            let num = parse_i32(&input);
            println!("{:?}", Solution::int_to_roman(num));
        }
        Commands::Convert {} => {
            let (s, num_rows) = parse_str_and_i32(&input);
            println!("{:?}", Solution::convert(s, num_rows));
        }
        Commands::Candy {} => {
            let ratings = parse_i32_list(&input);
            println!("{:?}", Solution::candy(ratings));
        }
        Commands::LetterCombinations {} => {
            let digits = parse_str(&input);
            println!("{:?}", Solution::letter_combinations(digits));
        }
        Commands::Combine {} => {
            let (n, k) = parse_two_i32(&input);
            println!("{:?}", Solution::combine(n, k));
        }
        Commands::CombinationSum {} => {
            let (candidates, target) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::combination_sum(candidates, target));
        }
        Commands::TotalNQueens {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::total_n_queens(n));
        }
        Commands::Exist {} => {
            let (board, word) = parse_2d_char_list_and_str(&input);
            println!("{:?}", Solution::exist(board, word));
        }
        Commands::SortedArrayToBst {} => {
            let nums = parse_i32_list(&input);
            let root = Solution::sorted_array_to_bst(nums);
            println!("{:?}", Tree { root });
        }
        Commands::IsPalindromeStr {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::is_palindrome_str(s));
        }
        Commands::AddTwoNumbers {} => {
            let (l1, l2) = parse_two_i32_list(&input);
            let h1 = SinglyLinkedList::from(l1).head;
            let h2 = SinglyLinkedList::from(l2).head;
            let output = SinglyLinkedList {
                head: Solution::add_two_numbers(h1, h2),
            };
            println!("{:?}", output);
        }
        Commands::MaxArea {} => {
            let heights = parse_i32_list(&input);
            println!("{:?}", Solution::max_area(heights));
        }
        Commands::ThreeSum {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::three_sum(nums));
        }
        Commands::Trap {} => {
            let height = parse_i32_list(&input);
            println!("{:?}", Solution::trap(height));
        }
        Commands::FullJustify {} => {
            let (words, max_width) = parse_str_list_and_i32(&input);
            println!("{:?}", Solution::full_justify(words, max_width));
        }
        Commands::MinSubArrayLen {} => {
            let (target, nums) = parse_i32_and_i32_list(&input);
            println!("{:?}", Solution::min_sub_array_len(target, nums));
        }
        Commands::LengthOfLongestSubstring {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::length_of_longest_substring(s));
        }
        Commands::IsValidSudoku {} => {
            let board = parse_2d_char_list(&input);
            println!("{:?}", Solution::is_valid_sudoku(board));
        }
        Commands::SpiralOrder {} => {
            let matrix = parse_2d_i32_list(&input);
            println!("{:?}", Solution::spiral_order(matrix));
        }
        Commands::RotateImage {} => {
            let mut matrix = parse_2d_i32_list(&input);
            Solution::rotate_image(&mut matrix);
            print!("[{:?}", matrix[0]);
            for row in matrix.iter().skip(1) {
                print!("\n {:?}", row);
            }
            println!("]");
        }
        Commands::SetZeroes {} => {
            let mut matrix = parse_2d_i32_list(&input);
            Solution::set_zeroes(&mut matrix);
            print!("[{:?}", matrix[0]);
            for row in matrix.iter().skip(1) {
                print!("\n {:?}", row);
            }
            println!("]");
        }
        Commands::GameOfLife {} => {
            let mut board = parse_2d_i32_list(&input);
            Solution::game_of_life(&mut board);
            print!("[{:?}", board[0]);
            for row in board.iter().skip(1) {
                print!("\n {:?}", row);
            }
            println!("]");
        }
        Commands::SummaryRanges {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::summary_ranges(nums));
        }
        Commands::MergeIntervals {} => {
            let intervals = parse_2d_i32_list(&input);
            println!("{:?}", Solution::merge_intervals(intervals));
        }
        Commands::FindSubstring {} => {
            let (s, words) = parse_str_and_str_list(&input);
            println!("{:?}", Solution::find_substring(s, words));
        }
        Commands::Insert {} => {
            let (intervals, new_interval) = parse_2d_i32_list_and_i32_list(&input);
            println!("{:?}", Solution::insert(intervals, new_interval));
        }
        Commands::FindMinArrowShots {} => {
            let points = parse_2d_i32_list(&input);
            println!("{:?}", Solution::find_min_arrow_shots(points));
        }
        Commands::MinWindow {} => {
            let (s, t) = parse_two_str(&input);
            println!("{:?}", Solution::min_window(s, t));
        }
        Commands::SimplifyPath {} => {
            let path = parse_str(&input);
            println!("{:?}", Solution::simplify_path(path));
        }
        Commands::ReverseBetween {} => {
            let (list, left, right) = parse_i32_list_and_two_i32(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::reverse_between(head, left, right)
                }
            );
        }
        Commands::MinStack {} => {
            let (commands, values) = parse_str_list_and_2d_i32_list(&input);
            let mut stack = MinStack::new();
            for (command, value) in commands.into_iter().zip(values.into_iter()) {
                match command.as_str() {
                    "MinStack" => {
                        print!("[null")
                    }
                    "push" => {
                        stack.push(value[0]);
                        print!(",null")
                    }
                    "pop" => {
                        stack.pop();
                        print!(",null")
                    }
                    "top" => {
                        print!(",{}", stack.top());
                    }
                    "getMin" => {
                        print!(",{}", stack.get_min());
                    }
                    _ => {
                        panic!("unknown input")
                    }
                }
            }
            println!("]")
        }
        Commands::EvalRpn {} => {
            let tokens = parse_str_list(&input);
            println!("{:?}", Solution::eval_rpn(tokens));
        }
        Commands::Calculate {} => {
            let s = parse_str(&input);
            println!("{:?}", Solution::calculate(s));
        }
        Commands::RemoveNthFromEnd {} => {
            let (list, n) = parse_i32_list_and_i32(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::remove_nth_from_end(head, n)
                }
            );
        }
        Commands::DeleteDuplicates {} => {
            let list = parse_i32_list(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::delete_duplicates(head)
                }
            );
        }
        Commands::RotateRight {} => {
            let (list, k) = parse_i32_list_and_i32(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::rotate_right(head, k)
                }
            );
        }
        Commands::Partition {} => {
            let (list, x) = parse_i32_list_and_i32(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::partition(head, x)
                }
            );
        }
        Commands::MaxDepth {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            let root = tree.root.clone();
            println!("{:?}", Solution::max_depth(root));
        }
        Commands::IsSameTree {} => {
            let (nodes1, nodes2) = parse_two_opt_i32_list(&input);
            let tree1 = Tree::from(nodes1);
            let tree2 = Tree::from(nodes2);
            let root1 = tree1.root.clone();
            let root2 = tree2.root.clone();
            println!("{:?}", Solution::is_same_tree(root1, root2));
        }
        Commands::InvertTree {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            let root = tree.root.clone();
            println!("{:?}", Solution::invert_tree(root));
        }
        Commands::IsSymmetric {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            let root = tree.root.clone();
            println!("{:?}", Solution::is_symmetric(root));
        }
        Commands::BuildTreeFromPreorderAndInorder {} => {
            let (preorder, inorder) = parse_two_i32_list(&input);
            println!(
                "{:?}",
                Solution::build_tree_from_preorder_and_inorder(preorder, inorder)
            );
        }
        Commands::BuildTreeFromInorderAndPostorder {} => {
            let (inorder, postorder) = parse_two_i32_list(&input);
            println!(
                "{:?}",
                Solution::build_tree_from_inorder_and_postorder(inorder, postorder)
            );
        }
        Commands::Flatten {} => {
            let nodes = parse_opt_i32_list(&input);
            let mut tree = Tree::from(nodes);
            Solution::flatten(&mut tree.root);
            println!("{:?}", tree);
        }
        Commands::CountNodes {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::count_nodes(tree.root));
        }
        Commands::SumNumbers {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::sum_numbers(tree.root));
        }
        Commands::MaxPathSum {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::max_path_sum(tree.root));
        }
        Commands::LowestCommonAncestor {} => {
            let (nodes, p, q) = parse_opt_i32_list_and_two_i32(&input);
            let tree = Tree::from(nodes);
            let p = tree.search(p);
            let q = tree.search(q);
            let ans = Solution::lowest_common_ancestor(tree.root, p, q);
            println!("{}", ans.unwrap().borrow().val);
        }
        Commands::RightSideView {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::right_side_view(tree.root));
        }
        Commands::LevelOrder {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::level_order(tree.root));
        }
        Commands::AverageOfLevels {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::average_of_levels(tree.root));
        }
        Commands::ZigzagLevelOrder {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::zigzag_level_order(tree.root));
        }
        Commands::GetMinimumDifference {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::get_minimum_difference(tree.root));
        }
        Commands::KthSmallest {} => {
            let (nodes, k) = parse_opt_i32_list_and_i32(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::kth_smallest(tree.root, k));
        }
        Commands::IsValidBst {} => {
            let nodes = parse_opt_i32_list(&input);
            let tree = Tree::from(nodes);
            println!("{:?}", Solution::is_valid_bst(tree.root));
        }
        Commands::NumIslands {} => {
            let grid = parse_2d_char_list(&input);
            println!("{:?}", Solution::num_islands(grid));
        }
        Commands::Solve {} => {
            let mut board = parse_2d_char_list(&input);
            Solution::solve(&mut board);
            print!("[{:?}", board[0]);
            for row in board.iter().skip(1) {
                print!("\n {:?}", row);
            }
            println!("]");
        }
        Commands::CalcEquation {} => {
            let (equations, values, queries) =
                parse_2d_str_list_and_f64_list_and_2d_str_list(&input);
            println!("{:?}", Solution::calc_equation(equations, values, queries));
        }
        Commands::CanFinish {} => {
            let (num_courses, prerequisites) = parse_i32_and_2d_i32_list(&input);
            println!("{:?}", Solution::can_finish(num_courses, prerequisites));
        }
        Commands::FindOrder {} => {
            let (num_courses, prerequisites) = parse_i32_and_2d_i32_list(&input);
            println!("{:?}", Solution::find_order(num_courses, prerequisites));
        }
        Commands::SnakesAndLadders {} => {
            let board = parse_2d_i32_list(&input);
            println!("{:?}", Solution::snakes_and_ladders(board));
        }
        Commands::MinMutation {} => {
            let (start, end, bank) = parse_two_str_and_str_list(&input);
            println!("{:?}", Solution::min_mutation(start, end, bank));
        }
        Commands::LadderLength {} => {
            let (begin_word, end_word, word_list) = parse_two_str_and_str_list(&input);
            println!(
                "{:?}",
                Solution::ladder_length(begin_word, end_word, word_list)
            );
        }
        Commands::SortList {} => {
            let list = parse_i32_list(&input);
            let head = SinglyLinkedList::from(list).head;
            println!(
                "{:?}",
                SinglyLinkedList {
                    head: Solution::sort_list(head)
                }
            );
        }
        Commands::FindWords {} => {
            let (board, words) = parse_2d_char_list_and_str_list(&input);
            println!("{:?}", Solution::find_words(board, words));
        }
        Commands::MaxSubArray {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::max_sub_array(nums));
        }
        Commands::MaxSubarraySumCircular {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::max_subarray_sum_circular(nums));
        }
        Commands::SearchInsert {} => {
            let (nums, target) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::search_insert(nums, target));
        }
        Commands::SearchMatrix {} => {
            let (matrix, target) = parse_2d_i32_list_and_i32(&input);
            println!("{:?}", Solution::search_matrix(matrix, target));
        }
        Commands::FindPeakElement {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::find_peak_element(nums));
        }
        Commands::SearchRotatedSortedArray {} => {
            let (nums, target) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::search_rotated_sorted_array(nums, target));
        }
        Commands::SearchRange {} => {
            let (nums, target) = parse_i32_list_and_i32(&input);
            println!("{:?}", Solution::search_range(nums, target));
        }
        Commands::FindMin {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::find_min(nums));
        }
        Commands::FindMedianSortedArrays {} => {
            let (nums1, nums2) = parse_two_i32_list(&input);
            println!("{:?}", Solution::find_median_sorted_arrays(nums1, nums2));
        }
        Commands::MergeKLists {} => {
            let lists = parse_2d_i32_list(&input);
            let lists = lists
                .into_iter()
                .map(|nodes| SinglyLinkedList::from(nodes).head)
                .collect();
            println!("{:?}", Solution::merge_k_lists(lists));
        }
        Commands::AddBinary {} => {
            let (a, b) = parse_two_str(&input);
            println!("{:?}", Solution::add_binary(a, b));
        }
        Commands::ReverseBits {} => {
            let n = parse_bits(&input);
            let rev_n = Solution::reverse_bits(n);
            println!("{rev_n} ({rev_n:032b})",);
        }
        Commands::HammingWeight {} => {
            let n = parse_i32(&input);
            println!("{:?}", Solution::hamming_weight(n));
        }
        Commands::SingleNumber {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::single_number(nums));
        }
        Commands::SingleNumber2 {} => {
            let nums = parse_i32_list(&input);
            println!("{:?}", Solution::single_number2(nums));
        }
    }
}
