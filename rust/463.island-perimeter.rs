/*
 * @lc app=leetcode id=463 lang=rust
 *
 * [463] Island Perimeter
 */

// @lc code=start
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        let r = grid.len();
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    perimeter += 4;
                    // up
                    match grid.get(row - 1) {
                        Some(r) if r[col] == 1 => {
                            perimeter -= 1;
                        }
                        _ => (),
                    }
                    // down
                    match grid.get(row + 1) {
                        Some(r) if r[col] == 1 => {
                            perimeter -= 1;
                        }
                        _ => (),
                    }
                    // left
                    match grid[row].get(col - 1) {
                        Some(g) if *g == 1 => {
                            perimeter -= 1;
                        }
                        _ => (),
                    }
                    // right
                    match grid[row].get(col + 1) {
                        Some(g) if *g == 1 => {
                            perimeter -= 1;
                        }
                        _ => (),
                    }
                }
            }
        }
        perimeter
    }
}
// @lc code=end
