/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut lp = String::new();
        fn find_palindrome(s: &str, mut l: usize, mut r: usize) -> String {
            while l >= 0 && r < s.len() && s.bytes().nth(l) == s.bytes().nth(r) {
                l -= 1;
                r += 1;
            }
            return s[l+1..r].to_string();
        }
        for i in 0..s.len() {
            let p1 = find_palindrome(&s, i, i);
            let p2 = find_palindrome(&s, i, i + 1);
            if p1.len() > lp.len() {
                lp = p1;
            }
            if p2.len() > lp.len() {
                lp = p2;
            }
        }
        return lp;
    }
}
// @lc code=end
