/*
 * @lc app=leetcode id=345 lang=rust
 *
 * [345] Reverse Vowels of a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let (mut left, mut right) = (0, s.len() - 1);
        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        while left < right {
            while left < right && !vowels.contains(&s[left]) {
                left += 1;
            }
            while left < right && !vowels.contains(&s[right]) {
                right -= 1;
            }
            if left < right {
                s.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        return s.iter().collect();
    }
}
// @lc code=end
