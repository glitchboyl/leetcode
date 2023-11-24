/*
 * @lc app=leetcode id=73 lang=rust
 *
 * [73] Set Matrix Zeroes
 */

// @lc code=start
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let entire_first_row_zero = matrix[0].contains(&0);
        let mut entire_first_column_zero = matrix[0][0] == 0;
        for i in 1..matrix.len() {
            entire_first_column_zero = entire_first_column_zero || matrix[i][0] == 0;
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in (0..matrix.len()).rev() {
            let entire_row_zero = matrix[i][0] == 0;
            if entire_first_column_zero || (i == 0 && entire_first_row_zero) {
                matrix[i][0] = 0;
            }
            for j in 1..matrix[0].len() {
                if entire_row_zero || matrix[0][j] == 0 || (i == 0 && entire_first_row_zero) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
// @lc code=end
