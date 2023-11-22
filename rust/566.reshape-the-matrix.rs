/*
 * @lc app=leetcode id=566 lang=rust
 *
 * [566] Reshape the Matrix
 */

// @lc code=start
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let (r, c) = (r as usize, c as usize);
        if m * n != r * c || (m == r && n == c) {
            return mat;
        }
        mat.into_iter()
            .flatten()
            .collect::<Vec<i32>>()
            .chunks(c)
            .map(|row| row.to_vec())
            .collect::<Vec<_>>()
    }
}
// @lc code=end
