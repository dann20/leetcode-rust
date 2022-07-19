/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 * Example 1:
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 * Example 2:
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn expand(l: usize, r: usize, s: &String) -> (usize, usize, usize) {
            let mut tmp_l = l;
            let mut tmp_r = r;
            let mut l = l;
            let mut r = r;
            if s.get(tmp_l..(tmp_l + 1)) != s.get(tmp_r..(tmp_r + 1)) {
                return (0, 0, 0);
            }
            while s.get(tmp_l..(tmp_l + 1)) == s.get(tmp_r..(tmp_r + 1)) {
                l = tmp_l;
                r = tmp_r;
                if tmp_l == 0 || tmp_r == s.len() - 1 {
                    break;
                }
                tmp_l -= 1;
                tmp_r += 1;
            }
            (l, r, r - l + 1)
        }

        let mut res: String = String::from("");
        for (idx, ch) in s.char_indices() {
            let (l1, r1, len1) = expand(idx, idx, &s);
            let (l2, r2, len2) = expand(idx, idx + 1, &s);
            match (len1 > len2, res.len() > len1 && res.len() > len2) {
                (_, true) => (),
                (true, false) => res = s.get(l1..(r1 + 1)).unwrap().to_string(),
                (false, false) => res = s.get(l2..(r2 + 1)).unwrap().to_string(),
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
    fn test_5() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "aba".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("asdksdfghbccbhgasf".to_string()),
            "ghbccbhg".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("qwiofvxcscxvf".to_string()),
            "fvxcscxvf".to_string()
        );
    }
}
