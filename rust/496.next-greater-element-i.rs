/*
 * @lc app=leetcode id=496 lang=rust
 *
 * [496] Next Greater Element I
 */

// @lc code=start
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut greater = HashMap::from([(nums2.last().unwrap(), -1)]);
        for i in (0..nums2.len() - 1).rev() {
            let mut n = nums2[i + 1];
            if nums2[i] > n {
                while n != -1 && n < nums2[i] {
                    n = greater[&n];
                }
            }
            greater.insert(&nums2[i], n);
        }
        nums1.iter().map(|n| greater[n]).collect()
    }
}
// @lc code=end
