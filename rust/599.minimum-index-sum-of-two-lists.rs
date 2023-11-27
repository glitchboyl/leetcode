/*
 * @lc app=leetcode id=599 lang=rust
 *
 * [599] Minimum Index Sum of Two Lists
 */

// @lc code=start
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut index_sum = list1.len() + list2.len();
        let mut restaurant = list2;
        let map: HashMap<_, usize> = list1.into_iter().zip(0 as usize..).collect();
        let mut k = 0;
        for j in 0..restaurant.len() {
            if let Some(i) = map.get(&restaurant[j]) {
                let idx_sum = i + j;
                if idx_sum <= index_sum {
                    if idx_sum < index_sum {
                        index_sum = idx_sum;
                        k = 0;
                    }
                    restaurant.swap(j, k);
                    k += 1;
                }
            }
        }
        restaurant.truncate(k);
        restaurant
    }
}
// @lc code=end
