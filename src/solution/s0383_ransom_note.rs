/**
 * [383] Ransom Note
 *
 * Given two strings ransomNote and magazine, return true if ransomNote can be constructed from magazine and false otherwise.
 * Each letter in magazine can only be used once in ransomNote.
 *
 * Example 1:
 * Input: ransomNote = "a", magazine = "b"
 * Output: false
 * Example 2:
 * Input: ransomNote = "aa", magazine = "ab"
 * Output: false
 * Example 3:
 * Input: ransomNote = "aa", magazine = "aab"
 * Output: true
 *
 * Constraints:
 *
 * 	1 <= ransomNote.length, magazine.length <= 10^5
 * 	ransomNote and magazine consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ransom-note/
// discuss: https://leetcode.com/problems/ransom-note/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut collections = magazine.clone();
        for c in ransom_note.chars() {
            if collections.len() == 0 {
                return false;
            }
            // match collections.find(c) {
            //     Some(idx) => _ = collections.remove(idx),
            //     None => return false,
            // }
            if let Some(idx) = collections.find(c) {
                collections.remove(idx);
            } else {
                    return false;
                }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383() {
        assert_eq!(Solution::can_construct(String::from("a"), String::from("b")), false);
        assert_eq!(Solution::can_construct(String::from("aa"), String::from("ab")), false);
        assert_eq!(Solution::can_construct(String::from("aa"), String::from("aab")), true);
        assert_eq!(Solution::can_construct(String::from("thisisaransomnote"), String::from("notenough")), false);
    }
}
