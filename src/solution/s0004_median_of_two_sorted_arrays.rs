/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *
 * Example 1:
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 * Example 2:
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
        use std::cmp::{max, min};

        let total: i32 = (nums1.len() + nums2.len()) as i32;
        let half: i32 = total / 2;

        // assign to arr_a whichever is shorter and perform binary search on it.
        let arr_a: Vec<i32>;
        let arr_b: Vec<i32>;
        if nums1.len() > nums2.len() {
            arr_a = nums2;
            arr_b = nums1;
        } else {
            arr_a = nums1;
            arr_b = nums2;
        }

        let mut l: i32 = 0;
        let mut r: i32 = (arr_a.len() as i32) - 1;

        loop {
            let m_a: i32 = ((l + r) as f64 / 2.0).floor() as i32;
            let m_b: i32 = half - m_a - 2;

            let left_a = if m_a >= 0 { arr_a[m_a as usize] } else { i32::MIN };
            let right_a = if (m_a + 1) < (arr_a.len() as i32) {
                arr_a[(m_a + 1) as usize]
            } else {
                i32::MAX
            };

            let left_b = if m_b >= 0 { arr_b[m_b as usize] } else { i32::MIN };
            let right_b = if (m_b + 1) < (arr_b.len() as i32) {
                arr_b[(m_b + 1) as usize]
            } else {
                i32::MAX
            };

            if left_a <= right_b && left_b <= right_a {
                if total % 2 == 0 {
                    return (((max(left_a, left_b) + min(right_a, right_b)) as f64) / 2.0);
                } else {
                    return min(right_a, right_b) as f64;
                }
            } else if left_a > right_b {
                r = m_a - 1;
            } else if left_b > right_a {
                l = m_a + 1;
            }
        }
        unreachable!();
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
            2.0 as f64
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5 as f64
        );
    }
}
