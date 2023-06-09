/**
 * [18] 4Sum
 *
 * Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
 *
 * 	0 <= a, b, c, d < n
 * 	a, b, c, and d are distinct.
 * 	nums[a] + nums[b] + nums[c] + nums[d] == target
 *
 * You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,0,-1,0,-2,2], target = 0
 * Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,2,2,2,2], target = 8
 * Output: [[2,2,2,2]]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 200
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/4sum/
// discuss: https://leetcode.com/problems/4sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut ans = Vec::new();
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() - 3 {
            if nums[i] > target && (nums[i] > 0 || target > 0) {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() - 2 {
                if nums[i] + nums[j] > target && (nums[i] + nums[j] > 0 || target > 0) {
                    break;
                }
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut l = nums.len() - 1;
                for k in j + 1..l {
                    if k > j + 1 && nums[k] == nums[k - 1] {
                        continue;
                    }
                    while k < l && nums[i] + nums[j] + nums[k] + nums[l] > target {
                        l -= 1;
                    }
                    if k == l {
                        break;
                    }
                    if nums[i] + nums[j] + nums[k] + nums[l] == target {
                        ans.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                    }
                }
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
    fn test_18() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
        assert_eq!(
            Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -294967296),
            Vec::<Vec<i32>>::new()
        );
    }
}
