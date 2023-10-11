/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy, mut profit) = (prices[0], 0);
        for i in 1..prices.len() {
            buy = buy.min(prices[i]);
            profit = profit.max(prices[i] - buy);
        }
        return profit;
    }
}
// @lc code=end
