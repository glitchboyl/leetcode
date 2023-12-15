/*
 * @lc app=leetcode id=680 lang=rust
 *
 * [680] Valid Palindrome II
 */

// @lc code=start
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            if s[left] != s[right] {
                if s[left + 1] == s[right] {
                    let (mut l, mut r) = (left + 1, right);
                    while l < r && s[l] == s[r] {
                        l += 1;
                        r -= 1;
                    }
                    if l >= r {
                        break;
                    }
                }
                if s[left] == s[right - 1] {
                    let (mut l, mut r) = (left, right - 1);
                    while l < r && s[l] == s[r] {
                        l += 1;
                        r -= 1;
                    }
                    if l >= r {
                        break;
                    }
                }
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
// @lc code=end
