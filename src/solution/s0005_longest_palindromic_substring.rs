/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest <span data-keyword="palindromic-string">palindromic</span> <span data-keyword="substring-nonempty">substring</span> in s.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn find_palindrome(chars: &Vec<char>, index: usize, center: bool) -> String {
            let mut left: i32 = index as i32;
            let mut right = if center { index } else { index + 1 };
            while left >= 0 && right < chars.len() {
                if chars[left as usize] != chars[right] {
                    break;
                }
                left = left - 1;
                right = right + 1;
            }
            chars[((left + 1) as usize)..right]
                .iter()
                .collect::<String>()
        }
        if s.len() <= 1 {
            return s;
        }
        let mut ans = "".to_string();
        let chars = s.chars().collect();
        for i in 0..s.len() {
            let ans1 = find_palindrome(&chars, i, true);
            if ans1.len() > ans.len() {
                ans = ans1;
            }
            let ans2 = find_palindrome(&chars, i, false);
            if ans2.len() > ans.len() {
                ans = ans2;
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
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
    }
}
