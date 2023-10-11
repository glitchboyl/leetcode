/*
 * @lc app=leetcode id=26 lang=javascript
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var removeDuplicates = function (nums) {
  // Solution 1:
  // let i = 0,
  //   k = 0,
  //   temp;
  // while (i < nums.length) {
  //   if (temp !== nums[i]) {
  //     temp = nums[i];
  //     i++;
  //     k++;
  //   } else {
  //     nums.splice(i, 1);
  //   }
  // }
  // return k;

  // Solution 2: slow and fast pointers
  let slow = 0,
    fast = 1;
  while (fast < nums.length) {
    if (nums[fast] !== nums[slow]) {
      nums[++slow] = nums[fast];
    }
    fast++;
  }
  return slow + 1;
};
// @lc code=end
