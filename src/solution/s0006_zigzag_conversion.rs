/**
 * [6] Zigzag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 * string convert(string s, int numRows);
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 	1 <= numRows <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/
// discuss: https://leetcode.com/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows >= s.len() as i32 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut ans = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        let seg_len = (num_rows * 2 - 2) as usize;
        for i in 0..num_rows as usize {
            let mut j = 0;
            while j < s.len() - i {
                ans.push(chars[i + j]);
                if i > 0 && i < num_rows - 1 && j + seg_len - i < s.len() {
                    ans.push(chars[j + seg_len - i]);
                }
                j += seg_len;
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
        assert_eq!(
            Solution::convert(String::from("A"), 1),
            String::from("A")
        );
    }
}
