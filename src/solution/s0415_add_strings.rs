use std::iter;

/**
 * [415] Add Strings
 *
 * Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.
 * You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.
 *
 * Example 1:
 *
 * Input: num1 = "11", num2 = "123"
 * Output: "134"
 *
 * Example 2:
 *
 * Input: num1 = "456", num2 = "77"
 * Output: "533"
 *
 * Example 3:
 *
 * Input: num1 = "0", num2 = "0"
 * Output: "0"
 *
 *
 * Constraints:
 *
 * 	1 <= num1.length, num2.length <= 10^4
 * 	num1 and num2 consist of only digits.
 * 	num1 and num2 don't have any leading zeros except for the zero itself.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-strings/
// discuss: https://leetcode.com/problems/add-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut num_vec: Vec<(u8, u8)> = Vec::new();
        let more: String;
        let less: String;

        if num1.len() >= num2.len() {
            more = num1;
            less = num2;
        } else {
            more = num2;
            less = num1;
        }

        for (a, b) in more
            .bytes()
            .rev()
            .zip(less.bytes().rev().chain(std::iter::repeat(48u8)))
        {
            num_vec.push((a, b));
        }

        let mut carry = 0;
        let mut res: String = String::new();
        for (a, b) in num_vec.iter() {
            let mut tmp = a + b - b'0' + carry;
            carry = 0;
            if tmp > b'9' {
                tmp -= 10u8;
                carry = 1;
            }
            let tmp_str = String::from(tmp as char);
            res = tmp_str + &res;
        }

        if carry == 1 {
            res = String::from("1") + &res;
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_415() {
        assert_eq!(
            Solution::add_strings("123".to_string(), "456".to_string()),
            "579".to_string()
        );
        assert_eq!(
            Solution::add_strings("11".to_string(), "123".to_string()),
            "134".to_string()
        );
        assert_eq!(
            Solution::add_strings("0".to_string(), "0".to_string()),
            "0".to_string()
        );
        assert_eq!(
            Solution::add_strings("456".to_string(), "77".to_string()),
            "533".to_string()
        );
        assert_eq!(
            Solution::add_strings("1".to_string(), "9".to_string()),
            "10".to_string()
        );
    }
}
