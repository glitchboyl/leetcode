/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = vec![vec![1]];
        for i in 1..num_rows {
            let mut row = vec![1];
            for j in 1..i {
                row.push(
                    triangle[(i - 1) as usize][(j - 1) as usize]
                        + triangle[(i - 1) as usize][j as usize],
                );
            }
            row.push(1);
            triangle.push(row);
        }
        return triangle;
    }
}
// @lc code=end
