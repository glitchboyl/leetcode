/*
 * @lc app=leetcode id=71 lang=javascript
 *
 * [71] Simplify Path
 */

// @lc code=start
/**
 * @param {string} path
 * @return {string}
 */
var simplifyPath = function (path) {
  return (
    "/" +
    path
      .split("/")
      .reduce((acc, p) => {
        if (p === "..") acc.pop();
        else if (p !== "" && p !== ".") acc.push(p);
        return acc;
      }, [])
      .join("/")
  );
};
// @lc code=end
