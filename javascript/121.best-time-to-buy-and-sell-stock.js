/*
 * @lc app=leetcode id=121 lang=javascript
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function (prices) {
  let buy = prices[0],
    profit = 0;
  for (let i = 1; i < prices.length; i++) {
    if (prices[i] - buy > profit) profit = prices[i] - buy;
    if (prices[i] < buy) buy = prices[i];
  }
  return profit;
};
// @lc code=end
