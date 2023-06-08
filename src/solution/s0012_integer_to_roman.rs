/**
 * [12] Integer to Roman
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
 * For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 * 	I can be placed before V (5) and X (10) to make 4 and 9.
 * 	X can be placed before L (50) and C (100) to make 40 and 90.
 * 	C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 * Given an integer, convert it to a roman numeral.
 *  
 * <strong class="example">Example 1:
 *
 * Input: num = 3
 * Output: "III"
 * Explanation: 3 is represented as 3 ones.
 *
 * <strong class="example">Example 2:
 *
 * Input: num = 58
 * Output: "LVIII"
 * Explanation: L = 50, V = 5, III = 3.
 *
 * <strong class="example">Example 3:
 *
 * Input: num = 1994
 * Output: "MCMXCIV"
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 3999
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-to-roman/
// discuss: https://leetcode.com/problems/integer-to-roman/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        use std::collections::HashMap;

        let mut roman = String::new();
        let mut num = num;
        loop {
            let (roman_str, roman_integer): (&str, i32) = match num {
                1000.. => ("M", 1000),
                900..=999 => ("CM", 900),
                500..=899 => ("D", 500),
                400..=499 => ("CD", 400),
                100..=399 => ("C", 100),
                90..=99 => ("XC", 90),
                50..=89 => ("L", 50),
                40..=49 => ("XL", 40),
                10..=39 => ("X", 10),
                9 => ("IX", 9),
                5..=8 => ("V", 5),
                4 => ("IV", 4),
                1..=3 => ("I", 1),
                _ => return roman,
            };
            num -= roman_integer;
            roman.push_str(roman_str);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
