/*
 * @lc app=leetcode id=350 lang=rust
 *
 * [350] Intersection of Two Arrays II
 */

// @lc code=start
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut nums_counters = HashMap::new();
        let mut result = Vec::new();
        for n in nums1 {
            nums_counters
                .entry(n)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for n in nums2 {
            match nums_counters.get(&n) {
                Some(&counter) if counter > 0 => {
                    nums_counters.entry(n).and_modify(|counter| *counter -= 1);
                    result.push(n);
                }
                _ => (),
            }
        }
        return result;
    }
}
// @lc code=end
