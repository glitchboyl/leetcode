/*
 * @lc app=leetcode id=38 lang=rust
 *
 * [38] Count and Say
 */

// @lc code=start
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        // Solution 1:
        // let mut digit_str = String::from("1");
        // for _ in 0..n - 1 {
        //     let mut s = String::new();
        //     let mut iter = digit_str.chars().enumerate();
        //     let mut p = iter.next().unwrap();
        //     while let Some(q) = iter.next() {
        //         if p.1 == q.1 {
        //             continue;
        //         }
        //         s = format!("{s}{}{}", q.0 - p.0, p.1);
        //         p = q;
        //     }
        //     digit_str = format!("{s}{}{}", digit_str.len() - p.0, p.1);
        // }
        // digit_str

        // Solution 2:
        let mut count = vec![1];
        for _ in 1..n {
            let mut say = vec![];
            let (mut s, mut f) = (0, 1);
            while f < count.len() {
                if count[s] != count[f] {
                    say.push((f - s) as u8);
                    say.push(count[s] as u8);
                    s = f;
                }
                f += 1;
            }
            say.push((f - s) as u8);
            say.push(count[s] as u8);
            count = say;
        }
        count
            .into_iter()
            .map(|d| (d + b'0') as char)
            .collect::<String>()
    }
}
// @lc code=end
