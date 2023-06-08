/**
 * [13] Roman to Integer
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 * 	I can be placed before V (5) and X (10) to make 4 and 9.
 * 	X can be placed before L (50) and C (100) to make 40 and 90.
 * 	C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 * Given a roman numeral, convert it to an integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "III"
 * Output: 3
 * Explanation: III = 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "LVIII"
 * Output: 58
 * Explanation: L = 50, V= 5, III = 3.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "MCMXCIV"
 * Output: 1994
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 15
 * 	s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
 * 	It is guaranteed that s is a valid roman numeral in the range [1, 3999].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/roman-to-integer/
// discuss: https://leetcode.com/problems/roman-to-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut prev_ch = ' ';
        let chars: Vec<char> = s.chars().collect();
        let mut integer = 0;
        for ch in chars {
            match ch {
                'I' => integer += 1,
                'V' => integer += if prev_ch == 'I' { 3 } else { 5 },
                'X' => integer += if prev_ch == 'I' { 8 } else { 10 },
                'L' => integer += if prev_ch == 'X' { 30 } else { 50 },
                'C' => integer += if prev_ch == 'X' { 80 } else { 100 },
                'D' => integer += if prev_ch == 'C' { 300 } else { 500 },
                'M' => integer += if prev_ch == 'C' { 800 } else { 1000 },
                _ => {}
            }
            prev_ch = ch;
        }
        integer
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
