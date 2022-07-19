/**
 * [290] Word Pattern
 *
 * Given a pattern and a string s, find if s follows the same pattern.
 * Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
 *
 * Example 1:
 *
 * Input: pattern = "abba", s = "dog cat cat dog"
 * Output: true
 *
 * Example 2:
 *
 * Input: pattern = "abba", s = "dog cat cat fish"
 * Output: false
 *
 * Example 3:
 *
 * Input: pattern = "aaaa", s = "dog cat cat dog"
 * Output: false
 *
 *
 * Constraints:
 *
 * 	1 <= pattern.length <= 300
 * 	pattern contains only lower-case English letters.
 * 	1 <= s.length <= 3000
 * 	s contains only lowercase English letters and spaces ' '.
 * 	s does not contain any leading or trailing spaces.
 * 	All the words in s are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-pattern/
// discuss: https://leetcode.com/problems/word-pattern/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;

        let mut pattern_map: HashMap<char, u32> = HashMap::new();
        let mut count = 1;
        let mut pattern_num = 0;
        for ch in pattern.chars() {
            if !pattern_map.contains_key(&ch) {
                pattern_map.insert(ch, count);
                count += 1;
            }
            pattern_num = pattern_map.get(&ch).unwrap() + pattern_num * 10;
        }

        let mut word_map: HashMap<&str, u32> = HashMap::new();
        count = 1;
        let mut word_num = 0;
        for word in s.split_whitespace() {
            if !word_map.contains_key(&word) {
                word_map.insert(word, count);
                count += 1;
            }
            word_num = word_map.get(&word).unwrap() + word_num * 10;
        }

        pattern_num == word_num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_290() {
        let pattern1 = String::from("abba");
        let str1 = String::from("dog cat cat dog");
        assert_eq!(Solution::word_pattern(pattern1, str1), true);

        let pattern2 = String::from("abba");
        let str2 = String::from("dog cat cat fish");
        assert_eq!(Solution::word_pattern(pattern2, str2), false);
    }
}
