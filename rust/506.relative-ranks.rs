/*
 * @lc app=leetcode id=506 lang=rust
 *
 * [506] Relative Ranks
 */

// @lc code=start
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut result = vec![String::new(); score.len()];
        let mut place = 1;

        // Solution 1:
        // let mut sorted_score = score.iter().enumerate().collect::<Vec<_>>();
        // sorted_score.sort_by(|a, b| b.1.cmp(a.1));
        // for (i, _) in sorted_score {
        //     result[i] = match place {
        //         1 => "Gold Medal".to_string(),
        //         2 => "Silver Medal".to_string(),
        //         3 => "Bronze Medal".to_string(),
        //         n => n.to_string(),
        //     };
        //     place += 1;
        // }

        // Solution 2:
        use std::collections::BinaryHeap;
        let mut score_heap = BinaryHeap::new();
        for (i, points) in score.iter().enumerate() {
            score_heap.push((points, i));
        }
        while let Some((_, i)) = score_heap.pop() {
            result[i] = match place {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                n => n.to_string(),
            };
            place += 1;
        }

        result
    }
}
// @lc code=end
