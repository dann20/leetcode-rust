/**
 * [412] Fizz Buzz
 *
 * Given an integer n, return a string array answer (1-indexed) where:
 *
 * 	answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
 * 	answer[i] == "Fizz" if i is divisible by 3.
 * 	answer[i] == "Buzz" if i is divisible by 5.
 * 	answer[i] == i (as a string) if none of the above conditions are true.
 *
 *
 * Example 1:
 * Input: n = 3
 * Output: ["1","2","Fizz"]
 * Example 2:
 * Input: n = 5
 * Output: ["1","2","Fizz","4","Buzz"]
 * Example 3:
 * Input: n = 15
 * Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
 *
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fizz-buzz/
// discuss: https://leetcode.com/problems/fizz-buzz/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        enum Result {
            Normal,
            Fizz,
            Buzz,
            FizzBuzz,
        }

        fn check_condition(idx: i32) -> Result {
            let num = idx + 1;
            let check: (i32, i32) = (num % 3, num % 5);
            match check {
                (0, 0) => Result::FizzBuzz,
                (0, _) => Result::Fizz,
                (_, 0) => Result::Buzz,
                _ => Result::Normal,
            }
        }

        let mut vec_result: Vec<String> = Vec::new();
        for idx in 0..n {
            let res = check_condition(idx);
            match res {
                Result::FizzBuzz => vec_result.push("FizzBuzz".to_string()),
                Result::Fizz => vec_result.push("Fizz".to_string()),
                Result::Buzz => vec_result.push("Buzz".to_string()),
                Result::Normal => vec_result.push((idx + 1).to_string()),
            }
        }
        vec_result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_412() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
