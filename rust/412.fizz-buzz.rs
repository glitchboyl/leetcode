/*
 * @lc app=leetcode id=412 lang=rust
 *
 * [412] Fizz Buzz
 */

// @lc code=start
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::new();
        for i in 1..=n {
            let fizz = if i % 3 == 0 { "Fizz" } else { "" };
            let buzz = if i % 5 == 0 { "Buzz" } else { "" };
            answer.push(if fizz.is_empty() && buzz.is_empty() {
                format!("{i}")
            } else {
                format!("{fizz}{buzz}")
            });
        }
        return answer;
    }
}
// @lc code=end
