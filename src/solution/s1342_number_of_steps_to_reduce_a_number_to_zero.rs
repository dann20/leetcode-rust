/**
 * [1342] Number of Steps to Reduce a Number to Zero
 *
 * Given an integer num, return the number of steps to reduce it to zero.
 * In one step, if the current number is even, you have to divide it by 2, otherwise, you have to subtract 1 from it.
 *
 * Example 1:
 *
 * Input: num = 14
 * Output: 6
 * Explanation:
 * Step 1) 14 is even; divide by 2 and obtain 7.
 * Step 2) 7 is odd; subtract 1 and obtain 6.
 * Step 3) 6 is even; divide by 2 and obtain 3.
 * Step 4) 3 is odd; subtract 1 and obtain 2.
 * Step 5) 2 is even; divide by 2 and obtain 1.
 * Step 6) 1 is odd; subtract 1 and obtain 0.
 *
 * Example 2:
 *
 * Input: num = 8
 * Output: 4
 * Explanation:
 * Step 1) 8 is even; divide by 2 and obtain 4.
 * Step 2) 4 is even; divide by 2 and obtain 2.
 * Step 3) 2 is even; divide by 2 and obtain 1.
 * Step 4) 1 is odd; subtract 1 and obtain 0.
 *
 * Example 3:
 *
 * Input: num = 123
 * Output: 12
 *
 *
 * Constraints:
 *
 * 	0 <= num <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
// discuss: https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut count: i32 = 0;
        let mut num: i32 = num;
        while num != 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            count += 1;
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1342() {
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
