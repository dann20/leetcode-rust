/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 * Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
 *
 * Example 1:
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 * Example 2:
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *
 * Constraints:
 *
 * 	1 <= num1.length, num2.length <= 200
 * 	num1 and num2 consist of digits only.
 * 	Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/multiply-strings/
// discuss: https://leetcode.com/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // handle edge cases to reduce computations.
        match (num1.as_str(), num2.as_str()) {
            ("0", _) | (_, "0") => return "0".to_string(),
            ("1", _) => return num2,
            (_, "1") => return num1,
            (_, _) => (),
        }

        let mut res_vec: Vec<u8> = vec![0; num1.len() + num2.len() + 1];
        for (i, b) in num2.bytes().rev().enumerate() {
            let b = b - 48u8;
            for (j, a) in num1.bytes().rev().enumerate() {
                let a = a - 48u8;
                let tmp = b * a;
                let idx = i + j;
                let (ones, tens) = (tmp % 10, tmp / 10);

                let tmp_ones = res_vec[idx] + ones;
                res_vec[idx] = tmp_ones % 10;
                res_vec[idx + 1] += tmp_ones / 10;

                let tmp_tens = res_vec[idx + 1] + tens;
                res_vec[idx + 1] = tmp_tens % 10;
                res_vec[idx + 2] += tmp_tens / 10;
            }
        }

        res_vec = res_vec.into_iter().rev().map(|x| x + 48u8).collect();
        String::from_utf8(res_vec)
            .unwrap()
            .trim_start_matches('0')
            .to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
        assert_eq!(
            Solution::multiply("78".to_string(), "12".to_string()),
            "936".to_string()
        );
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
        assert_eq!(
            Solution::multiply("98".to_string(), "9".to_string()),
            "882".to_string()
        );
        assert_eq!(
            Solution::multiply("56".to_string(), "87569".to_string()),
            "4903864".to_string()
        );
        assert_eq!(
            Solution::multiply("8742".to_string(), "375".to_string()),
            "3278250".to_string()
        );
        assert_eq!(
            Solution::multiply("498828660196".to_string(), "840477629533".to_string()),
            "419254329864656431168468".to_string()
        );
    }
}
