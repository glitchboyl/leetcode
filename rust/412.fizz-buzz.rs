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
            let mut ans = String::new();
            if i % 3 == 0 {
                ans += "Fizz";
            }
            if i % 5 == 0 {
                ans += "Buzz";
            }
            if ans.is_empty() {
                ans += &i.to_string();
            }
            answer.push(ans);
        }
        return answer;
    }
}
// @lc code=end
