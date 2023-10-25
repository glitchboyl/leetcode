/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut parentheses: Vec<String> = vec![];
        fn backtrack_insert(
            l: i32,
            r: i32,
            n: i32,
            parenthesis: String,
            parentheses: &mut Vec<String>,
        ) {
            if l == n && r == n {
                parentheses.push(parenthesis);
                return;
            }
            if l < n {
                backtrack_insert(l + 1, r, n, format!("{parenthesis}("), parentheses);
            }
            if r < l && r < n {
                backtrack_insert(l, r + 1, n, format!("{parenthesis})"), parentheses);
            }
        }
        backtrack_insert(0, 0, n, String::new(), &mut parentheses);
        parentheses
    }
}
// @lc code=end
