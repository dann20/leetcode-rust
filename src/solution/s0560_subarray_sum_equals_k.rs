/**
 * [560] Subarray Sum Equals K
 *
 * Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *
 * Example 1:
 * Input: nums = [1,1,1], k = 2
 * Output: 2
 * Example 2:
 * Input: nums = [1,2,3], k = 3
 * Output: 2
 *
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	-1000 <= nums[i] <= 1000
 * 	-10^7 <= k <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-sum-equals-k/
// discuss: https://leetcode.com/problems/subarray-sum-equals-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut track_map: HashMap<i32, i32> = HashMap::from([(0, 1)]);
        let mut count = 0;
        let mut current_sum = 0;

        for &num in nums.iter() {
            current_sum += num;
            let diff = current_sum - k;
            count += *track_map.get(&diff).unwrap_or(&0);
            if let Some(val) = track_map.get_mut(&current_sum) {
                *val += 1;
            } else {
                track_map.insert(current_sum, 1);
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_560() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
