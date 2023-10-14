/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut start, mut end): (usize, usize) = (0, 1);
        let bytes = s.as_bytes();
        fn find_palindrome(
            bytes: &[u8],
            mut l: i32,
            mut r: i32,
            start: &mut usize,
            end: &mut usize,
        ) {
            let len = bytes.len() as i32;
            while l >= 0 && r < len && bytes[l as usize] == bytes[r as usize] {
                l -= 1;
                r += 1;
            }
            if r - l - 1 > (*end - *start) as i32 {
                *end = r as usize;
                *start = (l + 1) as usize;
            }
        }
        for i in 1..s.len() {
            let i = i as i32;
            find_palindrome(bytes, i, i, &mut start, &mut end);
            find_palindrome(bytes, i - 1, i, &mut start, &mut end);
        }
        return s[start..end].to_string();
    }
}
// @lc code=end
