use crate::Solution;

impl Solution {
    /// # [26. Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer array `nums` sorted in **non-decreasing order** , remove the duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">**in-place** </a> such that each unique element appears only **once** . The **relative order**  of the elements should be kept the **same** . Then return the number of unique elements in `nums`.
    ///
    /// Consider the number of unique elements of `nums` to be `k`, to get accepted, you need to do the following things:
    ///
    /// - Change the array `nums` such that the first `k` elements of `nums` contain the unique elements in the order they were present in `nums` initially. The remaining elements of `nums` are not important as well as the size of `nums`.
    /// - Return `k`.
    ///
    /// **Custom Judge:**
    ///
    /// The judge will test your solution with the following code:
    ///
    /// ```txt
    /// int[] nums = [...]; // Input array
    /// int[] expectedNums = [...]; // The expected answer with correct length
    ///
    /// int k = removeDuplicates(nums); // Calls your implementation
    ///
    /// assert k == expectedNums.length;
    /// for (int i = 0; i < k; i++) {
    ///     assert nums[i] == expectedNums[i];
    /// }
    /// ```
    ///
    /// If all assertions pass, then your solution will be **accepted** .
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,1,2]
    /// Output: 2, nums = [1,2,_]
    /// Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
    /// It does not matter what you leave beyond the returned k (hence they are underscores).
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [0,0,1,1,1,2,2,3,3,4]
    /// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
    /// Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
    /// It does not matter what you leave beyond the returned k (hence they are underscores).
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 3 * 10^4`
    /// - `-100 <= nums[i] <= 100`
    /// - `nums` is sorted in **non-decreasing**  order.
    pub fn remove_duplicates1(nums: &mut Vec<i32>) -> i32 {
        let mut j = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[j] = nums[i];
                j += 1;
            }
        }
        j as i32
    }

    /// # [80. Remove Duplicates from Sorted Array II](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer array `nums` sorted in **non-decreasing order** , remove some duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">**in-place** </a> such that each unique element appears **at most twice** . The **relative order**  of the elements should be kept the **same** .
    ///
    /// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the **first part**  of the array `nums`. More formally, if there are `k` elements after removing the duplicates, then the first `k` elements of `nums`should hold the final result. It does not matter what you leave beyond the first`k`elements.
    ///
    /// Return `k` after placing the final result in the first `k` slots of `nums`.
    ///
    /// Do **not**  allocate extra space for another array. You must do this by **modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>**  with O(1) extra memory.
    ///
    /// **Custom Judge:**
    ///
    /// The judge will test your solution with the following code:
    ///
    /// ```txt
    /// int[] nums = [...]; // Input array
    /// int[] expectedNums = [...]; // The expected answer with correct length
    ///
    /// int k = removeDuplicates(nums); // Calls your implementation
    ///
    /// assert k == expectedNums.length;
    /// for (int i = 0; i < k; i++) {
    ///     assert nums[i] == expectedNums[i];
    /// }
    /// ```
    ///
    /// If all assertions pass, then your solution will be **accepted** .
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,1,1,2,2,3]
    /// Output: 5, nums = [1,1,2,2,3,_]
    /// Explanation: Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
    /// It does not matter what you leave beyond the returned k (hence they are underscores).
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [0,0,1,1,1,1,2,3,3]
    /// Output: 7, nums = [0,0,1,1,2,3,3,_,_]
    /// Explanation: Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
    /// It does not matter what you leave beyond the returned k (hence they are underscores).
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 3 * 10^4`
    /// - `-10^4 <= nums[i] <= 10^4`
    /// - `nums` is sorted in **non-decreasing**  order.
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        let mut cnt = 1;

        let mut i = 1;
        while i != nums.len() {
            if nums[i] != nums[i - 1] {
                cnt = 1;
            } else if cnt < 2 {
                cnt += 1;
            } else {
                break;
            }
            i += 1;
        }

        let mut back = i;
        for i in i + 1..nums.len() {
            if nums[i] != nums[i - 1] {
                cnt = 1;
                nums[back] = nums[i];
                back += 1;
            } else if cnt < 2 {
                cnt += 1;
                nums[back] = nums[i];
                back += 1;
            }
        }
        back as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_1() {
        let mut nums = vec![1, 1, 2];
        let expected_nums = [1, 2];
        let k = Solution::remove_duplicates1(&mut nums);
        assert_eq!(nums[0..k as usize], expected_nums);
    }

    #[test]
    fn example1_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected_nums = [0, 1, 2, 3, 4];
        let k = Solution::remove_duplicates1(&mut nums);
        assert_eq!(nums[0..k as usize], expected_nums);
    }

    #[test]
    fn example1_3() {
        let mut nums = vec![0];
        let expected_nums = [0];
        let k = Solution::remove_duplicates1(&mut nums);
        assert_eq!(nums[0..k as usize], expected_nums);
    }

    #[test]
    fn example2_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let expected_nums = [1, 1, 2, 2, 3];
        let k = Solution::remove_duplicates2(&mut nums);
        assert_eq!(nums[0..k as usize], expected_nums);
    }

    #[test]
    fn example2_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let expected_nums = [0, 0, 1, 1, 2, 3, 3];
        let k = Solution::remove_duplicates2(&mut nums);
        assert_eq!(nums[0..k as usize], expected_nums);
    }
}
