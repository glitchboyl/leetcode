/*
 * @lc app=leetcode id=27 lang=javascript
 *
 * [27] Remove Element
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} val
 * @return {number}
 */
var removeElement = function (nums, val) {
  // Solution 1:
  // let i = 0;
  // while (i < nums.length) {
  //   if (nums[i] === val) nums.splice(i, 1);
  //   else i++;
  // }
  // return nums.length;

  // Solution 2: slow and fast pointers
  let slow = 0,
    fast = 0;
  while (fast < nums.length) {
    if (nums[fast] !== val) {
      let temp = nums[slow];
      nums[slow++] = nums[fast];
      nums[fast] = temp;
    }
    fast++;
  }
  return slow;
};
// @lc code=end
