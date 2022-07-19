/**
 * [15] 3Sum
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
 * Notice that the solution set must not contain duplicate triplets.
 *
 * Example 1:
 * Input: nums = [-1,0,1,2,-1,-4]
 * Output: [[-1,-1,2],[-1,0,1]]
 * Example 2:
 * Input: nums = []
 * Output: []
 * Example 3:
 * Input: nums = [0]
 * Output: []
 *
 * Constraints:
 *
 * 	0 <= nums.length <= 3000
 * 	-10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len: usize = nums.len();
        if len < 3 {
            let empty: Vec<Vec<i32>> = Vec::new();
            return empty;
        }

        use std::cmp::Ordering;

        let mut nums = nums;
        nums.sort();

        let mut res: Vec<Vec<i32>> = Vec::new();

        for idx in 0..(len - 2) {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                continue;
            }

            let mut l = idx + 1;
            let mut r = nums.len() - 1;

            while l < r {
                match (nums[idx] + nums[l] + nums[r]).cmp(&0) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        res.push(vec![nums[idx], nums[l], nums[r]]);
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                }
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        let ans1 = [[-1, -1, 2], [-1, 0, 1]];
        let res1 = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(
            res1,
            ans1.iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>()
        );

        let ans2 = [[-2, -1, 3], [-1, 0, 1]];
        let res2 = Solution::three_sum(vec![3, 0, -2, -1, 1]);
        assert_eq!(
            res2,
            ans2.iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>()
        );

        let ans3 = [[0, 0, 0]];
        let res3 = Solution::three_sum(vec![0, 0, 0, 0]);
        assert_eq!(
            res3,
            ans3.iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>()
        );
    }
}
