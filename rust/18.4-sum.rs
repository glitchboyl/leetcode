/*
 * @lc app=leetcode id=18 lang=rust
 *
 * [18] 4Sum
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut quadruplets = vec![];
        if nums.len() >= 4 {
            let mut ordered_nums = nums;
            ordered_nums.sort();
            let len = ordered_nums.len();
            for a in 0..len - 3 {
                if a > 0 && ordered_nums[a] == ordered_nums[a - 1] {
                    continue;
                }
                let sub_a = target - ordered_nums[a];
                for b in a + 1..len - 2 {
                    if b > a + 1 && ordered_nums[b] == ordered_nums[b - 1] {
                        continue;
                    }
                    let sub_b = if let Some(sub) = sub_a.checked_sub(ordered_nums[b]) {
                        sub
                    } else {
                        continue;
                    };
                    let (mut c, mut d) = (b + 1, len - 1);
                    while c < d {
                        let sum = ordered_nums[c] + ordered_nums[d];
                        if sum == sub_b {
                            quadruplets.push(vec![
                                ordered_nums[a],
                                ordered_nums[b],
                                ordered_nums[c],
                                ordered_nums[d],
                            ]);
                            c += 1;
                            d -= 1;
                            while ordered_nums[c] == ordered_nums[c - 1] && c < d {
                                c += 1;
                            }
                        } else if sum < sub_b {
                            c += 1;
                            while ordered_nums[c] == ordered_nums[c - 1] && c < d {
                                c += 1;
                            }
                        } else {
                            d -= 1;
                        }
                    }
                }
            }
        }
        return quadruplets;
    }
}
// @lc code=end
