/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut combinations: Vec<String> = vec![];
        if digits.len() > 0 {
            let map = [
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
                vec!['j', 'k', 'l'],
                vec!['m', 'n', 'o'],
                vec!['p', 'q', 'r', 's'],
                vec!['t', 'u', 'v'],
                vec!['w', 'x', 'y', 'z'],
            ];
            fn backtrack_insert(
                next_digit: &str,
                map: &[Vec<char>; 8],
                combination: String,
                combinations: &mut Vec<String>,
            ) {
                if let Some(d) = next_digit.chars().nth(0) {
                    for c in map[d.to_digit(10).unwrap() as usize - 2].iter() {
                        backtrack_insert(
                            &next_digit[1..],
                            map,
                            format!("{combination}{c}"),
                            combinations,
                        );
                    }
                } else {
                    combinations.push(combination);
                }
            }
            backtrack_insert(&digits, &map, String::new(), &mut combinations);
        }
        return combinations;
    }
}
// @lc code=end
