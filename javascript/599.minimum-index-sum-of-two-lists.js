/*
 * @lc app=leetcode id=599 lang=javascript
 *
 * [599] Minimum Index Sum of Two Lists
 */

// @lc code=start
/**
 * @param {string[]} list1
 * @param {string[]} list2
 * @return {string[]}
 */
var findRestaurant = function (list1, list2) {
  const map = new Map(list1.map((str, i) => [str, i]));
  let minIndexSum = list1.length + list2.length,
    k = 0;
  for (let j = 0; j < list2.length; j++) {
    if (map.has(list2[j])) {
      const indexSum = j + map.get(list2[j]);
      if (indexSum <= minIndexSum) {
        if (indexSum < minIndexSum) {
          minIndexSum = indexSum;
          k = 0;
        }
        [list2[j], list2[k]] = [list2[k], list2[j]];
        k++;
      }
    }
  }
  return list2.slice(0, k);
};
// @lc code=end
