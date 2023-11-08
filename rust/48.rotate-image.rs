/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
 */

// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
            matrix[i].reverse();
        }
    }
}
// @lc code=end
