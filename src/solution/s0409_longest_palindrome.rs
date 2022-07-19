/**
 * [409] Longest Palindrome
 *
 * Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
 * Letters are case sensitive, for example, "Aa" is not considered a palindrome here.
 *
 * Example 1:
 *
 * Input: s = "abccccdd"
 * Output: 7
 * Explanation:
 * One longest palindrome that can be built is "dccaccd", whose length is 7.
 *
 * Example 2:
 *
 * Input: s = "a"
 * Output: 1
 *
 * Example 3:
 *
 * Input: s = "bb"
 * Output: 2
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 2000
 * 	s consists of lowercase and/or uppercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindrome/
// discuss: https://leetcode.com/problems/longest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;

        let mut count = 0;
        let mut flag = false;
        let mut table: HashMap<char, i32> = HashMap::new();
        for ch in s.chars() {
            let counter = table.entry(ch).or_insert(0);
            *counter += 1;
        }

        for value in table.values() {
            if *value % 2 == 0 {
                count += *value;
            } else if !flag {
                count += *value;
                flag = true;
            } else {
                count += (*value - 1);
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
    fn test_409() {
        assert_eq!(Solution::longest_palindrome(String::from("abccccdd")), 7);
        assert_eq!(Solution::longest_palindrome(String::from("aAccccdd")), 7);
        assert_eq!(Solution::longest_palindrome(String::from("a")), 1);
        assert_eq!(Solution::longest_palindrome(String::from("bb")), 2);
        assert_eq!(Solution::longest_palindrome(String::from("ccc")), 3);
    }
}
