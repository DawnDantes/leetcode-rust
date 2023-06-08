/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 *  
 * Constraints:
 *
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use std::cmp::min;

        fn get_kth_element(nums1: &Vec<i32>, nums2: &Vec<i32>, k: usize) -> i32 {
            let mut start1 = 0;
            let mut start2 = 0;
            let mut k = k;
            loop {
                if start1 == nums1.len() {
                    return nums2[start2 + k - 1];
                }
                if start2 == nums2.len() {
                    return nums1[start1 + k - 1];
                }
                if k == 1 {
                    return min(nums1[start1], nums2[start2]);
                }
                let step = k / 2;
                let next_start1 = min(start1 + step, nums1.len()) - 1;
                let next_start2 = min(start2 + step, nums2.len()) - 1;
                if nums1[next_start1] <= nums2[next_start2] {
                    k -= next_start1 - start1 + 1;
                    start1 = next_start1 + 1;
                } else {
                    k -= next_start2 - start2 + 1;
                    start2 = next_start2 + 1;
                }
            }
        }
        let len = nums1.len() + nums2.len();
        let k = (len + 1) / 2;
        if len % 2 == 0 {
            ((get_kth_element(&nums1, &nums2, k) + get_kth_element(&nums1, &nums2, k + 1)) as f64)
                / 2.0
        } else {
            get_kth_element(&nums1, &nums2, k) as f64
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0f64
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5f64
        );
    }
}
