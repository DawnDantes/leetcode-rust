/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest <span data-keyword="substring-nonempty">substring</span> without repeating characters.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::{cmp::max, collections::HashMap};

        let mut map = HashMap::<char, usize>::new();
        let mut ans = 0;
        let mut left: usize = 0;
        let chars: Vec<char> = s.chars().collect();
        for (index, ch) in chars.iter().enumerate() {
            match map.get(&ch) {
                Some(i) => {
                    left = max(left, *i + 1);
                }
                None => {}
            }
            map.insert(*ch, index);
            ans = max(ans, (index - left + 1) as i32);
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abba")),
            2
        );
    }
}
