/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
 * A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png" style="width: 300px; height: 243px;" />
 *  
 * <strong class="example">Example 1:
 *
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 * <strong class="example">Example 2:
 *
 * Input: digits = ""
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: digits = "2"
 * Output: ["a","b","c"]
 *
 *  
 * Constraints:
 *
 * 	0 <= digits.length <= 4
 * 	digits[i] is a digit in the range ['2', '9'].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let letters_map = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        digits
            .bytes()
            .fold(vec!["".to_string()], |mut init, digit| {
                let mut item = Vec::new();
                let letters = letters_map[(digit - b'0') as usize]
                    .chars()
                    .collect::<Vec<_>>();
                while let Some(val) = init.pop() {
                    letters.iter().for_each(|letter| {
                        item.push(format!("{}{}", val, letter));
                    })
                }
                item
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["cd", "ce", "cf", "bd", "be", "bf", "ad", "ae", "af"]
        );
        assert_eq!(Solution::letter_combinations("".to_string()), vec![""; 0]);
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }
}
