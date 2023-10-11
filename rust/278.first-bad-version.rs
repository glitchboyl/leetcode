/*
 * @lc app=leetcode id=278 lang=rust
 *
 * [278] First Bad Version
 */

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        // Solution 1: recursion
        // if n != 1 && self.isBadVersion(n - 1) {
        //     return Self::first_bad_version(&self, n - 1);
        // }

        // Solution 2: binary search
        if n != 1 {
            let (mut left, mut right) = (1, n);
            while left <= right {
                let mid = left + (right - left) / 2;
                if self.isBadVersion(mid) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return left;
        }

        return n;
    }
}
// @lc code=end