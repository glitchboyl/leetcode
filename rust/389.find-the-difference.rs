/*
 * @lc app=leetcode id=389 lang=rust
 *
 * [389] Find the Difference
 */

// @lc code=start
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        // Solution 1:
        // let mut chars = vec![0; 26];
        // for c in t.bytes() {
        //     chars[(c - b'a') as usize] += 1;
        // }
        // for c in s.bytes() {
        //     chars[(c - b'a') as usize] -= 1;
        // }
        // return (chars.iter().position(|c| *c == 1).unwrap() as u8 + b'a') as char;

        // Solution 2: using XOR
        let mut diff = 0;
        for c in t.bytes() {
            diff ^= c;
        }
        for c in s.bytes() {
            diff ^= c;
        }
        return diff as char;
    }
}
// @lc code=end
