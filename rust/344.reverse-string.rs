/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 */

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // Solution 1:
        s.reverse();

        // Solution 2: two pointers
        // let (mut left, mut right): (usize, usize) = (0, s.len() - 1);
        // while left < right {
        //     s.swap(left, right);
        //     left += 1;
        //     right -= 1;
        // }
    }
}
// @lc code=end
