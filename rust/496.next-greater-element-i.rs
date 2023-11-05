/*
 * @lc app=leetcode id=496 lang=rust
 *
 * [496] Next Greater Element I
 */

// @lc code=start
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut greater_map = HashMap::from([(nums2.last().unwrap(), -1)]);
        for i in (0..nums2.len() - 1).rev() {
            let mut next_greater = if nums2[i + 1] > nums2[i] {
                nums2[i + 1]
            } else {
                let mut m = greater_map[&nums2[i + 1]];
                loop {
                    if m == -1 || m > nums2[i] {
                        break m;
                    }
                    m = greater_map[&m]
                }
            };
            greater_map.insert(&nums2[i], next_greater);
        }
        nums1.iter().map(|n| greater_map[n]).collect()
    }
}
// @lc code=end
