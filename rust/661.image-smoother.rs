/*
 * @lc app=leetcode id=661 lang=rust
 *
 * [661] Image Smoother
 */

// @lc code=start
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();
        let mut smoother = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut cell = img[i][j];
                let mut c = 1;
                if i > 0 {
                    cell += img[i - 1][j];
                    c += 1;
                    if j > 0 {
                        cell += img[i - 1][j - 1];
                        c += 1;
                    }
                    if j < n - 1 {
                        cell += img[i - 1][j + 1];
                        c += 1
                    }
                }
                if i < m - 1 {
                    cell += img[i + 1][j];
                    c += 1;
                    if j > 0 {
                        cell += img[i + 1][j - 1];
                        c += 1;
                    }
                    if j < n - 1 {
                        cell += img[i + 1][j + 1];
                        c += 1
                    }
                }
                if j > 0 {
                    cell += img[i][j - 1];
                    c += 1;
                }
                if j < n - 1 {
                    cell += img[i][j + 1];
                    c += 1
                }
                smoother[i][j] = cell / c;
            }
        }
        smoother
    }
}
// @lc code=end
