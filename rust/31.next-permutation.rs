/*
 * @lc app=leetcode id=31 lang=rust
 *
 * [31] Next Permutation
 */

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
        let mut l = nums.len() - 1;

        if l == 0 {
            return;
        }

        // 1. Find the largest index k such that a[k] < a[k + 1].
        // If no such index exists, the permutation is the last permutation.
        let mut k = l;
        while k > 0 {
            let found = nums[k] > nums[k - 1];
            k -= 1;
            if found {
                break;
            }
            if k == 0 {
                nums.sort();
                return;
            }
        }

        // 2. Find the largest index l greater than k such that a[k] < a[l].
        while nums[l] <= nums[k] {
            l -= 1;
        }

        // 3. Swap the value of a[k] with that of a[l].
        nums.swap(k, l);

        // 4. Reverse the sequence from a[k + 1] up to and including the final element a[n].
        nums[k + 1..].reverse();
    }
}
// @lc code=end
