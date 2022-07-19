/**
 * [75] Sort Colors
 *
 * Given an array nums with n objects colored red, white, or blue, sort them <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
 * We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
 * You must solve this problem without using the library's sort function.
 *
 * Example 1:
 *
 * Input: nums = [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 *
 * Example 2:
 *
 * Input: nums = [2,0,1]
 * Output: [0,1,2]
 *
 *
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 300
 * 	nums[i] is either 0, 1, or 2.
 *
 *
 * Follow up: Could you come up with a one-pass algorithm using only constant extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-colors/
// discuss: https://leetcode.com/problems/sort-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();

        let mut l = 0;
        let mut h = n;
        let mut count_1s = 0;

        while h > 0 && l + count_1s <= h - 1 {
            match nums[l + count_1s] {
                0 => {
                    nums.insert(l, 0);
                    nums.remove(l + count_1s + 1);
                    l += 1;
                }
                1 => count_1s += 1,
                2 => {
                    nums.insert(h, 2);
                    nums.remove(l + count_1s);
                    h -= 1;
                }
                _ => unreachable!(),
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_75() {
        let mut exp1 = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut exp1);
        assert_eq!(exp1, vec![0, 0, 1, 1, 2, 2]);

        let mut exp2 = vec![2, 0, 1];
        Solution::sort_colors(&mut exp2);
        assert_eq!(exp2, vec![0, 1, 2]);

        let mut exp3 = vec![1, 2, 1, 1, 0, 0, 0, 2];
        Solution::sort_colors(&mut exp3);
        assert_eq!(exp3, vec![0, 0, 0, 1, 1, 1, 2, 2]);

        let mut exp4 = vec![1];
        Solution::sort_colors(&mut exp4);
        assert_eq!(exp4, vec![1]);

        let mut exp5 = vec![2, 2];
        Solution::sort_colors(&mut exp5);
        assert_eq!(exp5, vec![2, 2]);
    }
}
