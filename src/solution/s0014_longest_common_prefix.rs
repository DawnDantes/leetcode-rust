/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 * If there is no common prefix, return an empty string "".
 *  
 * <strong class="example">Example 1:
 *
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 * <strong class="example">Example 2:
 *
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *  
 * Constraints:
 *
 * 	1 <= strs.length <= 200
 * 	0 <= strs[i].length <= 200
 * 	strs[i] consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-prefix/
// discuss: https://leetcode.com/problems/longest-common-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs[0].chars()
            .take(
                strs.iter()
                    .skip(1)
                    .map(|word| {
                        word.chars()
                            .zip(strs[0].chars())
                            .take_while(|(l, r)| l == r)
                            .count()
                    })
                    .min()
                    .unwrap_or_else(|| strs[0].len()),
            )
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "a".to_string()
            ]),
            "a"
        );
    }
}
