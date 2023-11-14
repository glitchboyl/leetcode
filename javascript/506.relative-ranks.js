/*
 * @lc app=leetcode id=506 lang=javascript
 *
 * [506] Relative Ranks
 */

// @lc code=start
/**
 * @param {number[]} score
 * @return {string[]}
 */
var findRelativeRanks = function (score) {
  const sortedScore = score.map((s, i) => [s, i]);
  sortedScore.sort((a, b) => b[0] - a[0]);
  const ranks = new Array(score.length);
  sortedScore.forEach(([_, i], place) => {
    ranks[i] = 
      place === 0
        ? "Gold Medal"
        : place === 1
        ? "Silver Medal"
        : place === 2
        ? "Bronze Medal"
        : `${place + 1}`;
  });
  return ranks;
};
// @lc code=end
