/*
 * @lc app=leetcode id=409 lang=rust
 *
 * [409] Longest Palindrome
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut letters_map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            letters_map.entry(c).and_modify(|t| *t += 1).or_insert(1);
        }
        let mut has_odd = false;
        return letters_map.into_values().fold(0, |acc, t| {
            has_odd = has_odd || t % 2 == 1;
            return acc + (t / 2) * 2;
        }) + if has_odd { 1 } else { 0 };
    }
}
// @lc code=end
