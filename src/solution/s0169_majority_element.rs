/**
 * [169] Majority Element
 *
 * Given an array nums of size n, return the majority element.
 * The majority element is the element that appears more than &lfloor;n / 2&rfloor; times. You may assume that the majority element always exists in the array.
 *
 * Example 1:
 * Input: nums = [3,2,3]
 * Output: 3
 * Example 2:
 * Input: nums = [2,2,1,1,1,2,2]
 * Output: 2
 *
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 5 * 10^4
 * 	-10^9 <= nums[i] <= 10^9
 *
 *
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element/
// discuss: https://leetcode.com/problems/majority-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for val in nums.iter() {
            let count = map.entry(*val).or_insert(0);
            *count += 1;
        }
        let result = map.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k);
        *result.unwrap()
    }

    pub fn majority_element_follow_up(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let mut result: i32 = nums[0];
        for &val in nums.iter() {
            if val == result {
                count += 1;
            } else {
                count -= 1;
            }
            match count < 0 {
                true => {
                    result = val;
                    count = 1;
                }
                false => (),
            }
        }
        result
    }

    pub fn majority_element_solution(nums: Vec<i32>) -> i32 {
        // BEST
        let mut count: u32 = 0;
        let mut result: i32 = nums[0];
        for &val in nums.iter() {
            if count == 0 {
                result = val;
            }
            match val == result {
                true => count += 1,
                false => count -= 1,
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_169() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
        assert_eq!(Solution::majority_element(vec![10, 9, 9, 9, 10]), 9);
    }

    #[test]
    fn test_169_follow_up() {
        assert_eq!(Solution::majority_element_follow_up(vec![3, 2, 3]), 3);
        assert_eq!(
            Solution::majority_element_follow_up(vec![2, 2, 1, 1, 1, 2, 2]),
            2
        );
        assert_eq!(
            Solution::majority_element_follow_up(vec![10, 9, 9, 9, 10]),
            9
        );
    }

    #[test]
    fn test_169_solution() {
        assert_eq!(Solution::majority_element_solution(vec![3, 2, 3]), 3);
        assert_eq!(
            Solution::majority_element_solution(vec![2, 2, 1, 1, 1, 2, 2]),
            2
        );
        assert_eq!(
            Solution::majority_element_solution(vec![10, 9, 9, 9, 10]),
            9
        );
    }
}
