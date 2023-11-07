/*
 * @lc app=leetcode id=45 lang=javascript
 *
 * [45] Jump Game II
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var jump = function (nums) {
  let steps = 0;
  let goal = nums.length - 1;

  // Solution 1:
  // let i = 0;
  // outer: while (i < goal) {
  //   steps++;
  //   let f = 0,
  //     farther = 0;
  //   for (let j = 1; j <= nums[i]; j++) {
  //     let n = i + j;
  //     if (n >= goal) break outer;
  //     const end = n + nums[n];
  //     if (end > farther) {
  //       f = n;
  //       farther = end;
  //     }
  //   }
  //   i = f;
  // }

  // Solution 2:
  let current = 0,
    farther = 0;
  for (let i = 0; i < goal; i++) {
    farther = Math.max(farther, i + nums[i]);
    if (i === current) {
      steps++;
      current = farther;
    }
  }
  
  return steps;
};
// @lc code=end
