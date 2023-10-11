/*
 * @lc app=leetcode id=349 lang=rust
 *
 * [349] Intersection of Two Arrays
 */

// @lc code=start
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        let mut nums_set: HashSet<i32> = HashSet::from_iter(nums1.into_iter());
        let mut result = Vec::new();
        for n in nums2 {
            if nums_set.contains(&n) {
                result.push(n);
                nums_set.remove(&n);
            }
        }
        return result;
    }
}
// @lc code=end
