/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in 0..n {
            nums1[(m + i) as usize] = nums2[i as usize];
        }
        nums1.sort_unstable();

        // let (mut i, mut j, mut k, mut d): (usize, usize, usize, usize) = (0, 0, 0, 0);
        // while (j as i32) < m && (k as i32) < n {
        //     if nums2[k] < nums1[j + d] {
        //         nums1.insert(i, nums2[k]);
        //         nums1.pop();
        //         k += 1;
        //         d += 1;
        //     } else {
        //         j += 1
        //     }
        //     i += 1;
        // }
        // while (k as i32) < n {
        //     nums1[i] = nums2[k];
        //     i += 1;
        //     k += 1;
        // }
    }
}
// @lc code=end
