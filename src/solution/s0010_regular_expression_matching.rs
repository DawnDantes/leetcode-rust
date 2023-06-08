/**
 * [10] Regular Expression Matching
 *
 * Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
 *
 * 	'.' Matches any single character.​​​​
 * 	'*' Matches zero or more of the preceding element.
 *
 * The matching should cover the entire input string (not partial).
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 20
 * 	1 <= p.length <= 20
 * 	s contains only lowercase English letters.
 * 	p contains only lowercase English letters, '.', and '*'.
 * 	It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/regular-expression-matching/
// discuss: https://leetcode.com/problems/regular-expression-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m: usize = s.len();
        let n: usize = p.len();
        let mut dp: Vec<Vec<bool>> = vec![vec![Default::default(); n + 1]; m + 1];
        dp[0][0] = true;
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        for j in 1..n + 1 {
            if p[j - 1] == '*' {
                dp[0][j] = dp[0][j - 2];
            }
        }
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                if p[j - 1] == '*' {
                    if s[i - 1] != p[j - 2] && p[j - 2] != '.' {
                        dp[i][j] = dp[i][j - 2];
                    } else {
                        dp[i][j] = dp[i][j - 2] | dp[i - 1][j];
                    }
                } else if s[i - 1] == p[j - 1] || p[j - 1] == '.' {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
        dp[m][n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        assert!(!Solution::is_match(String::from("aa"), String::from("a")));
        assert!(Solution::is_match(String::from("aa"), String::from("a*")));
        assert!(Solution::is_match(String::from("ab"), String::from(".*")));
    }
}
