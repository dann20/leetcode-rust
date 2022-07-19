/**
 * [763] Partition Labels
 *
 * You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.
 * Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.
 * Return a list of integers representing the size of these parts.
 *
 * Example 1:
 *
 * Input: s = "ababcbacadefegdehijhklij"
 * Output: [9,7,8]
 * Explanation:
 * The partition is "ababcbaca", "defegde", "hijhklij".
 * This is a partition so that each letter appears in at most one part.
 * A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.
 *
 * Example 2:
 *
 * Input: s = "eccbbbbdec"
 * Output: [10]
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-labels/
// discuss: https://leetcode.com/problems/partition-labels/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        use std::collections::HashMap;
        use std::cmp::max;

        let mut map: HashMap<char, usize> = HashMap::new();
        let mut vec: Vec<char> = Vec::new();
        for (idx, ch) in s.char_indices() {
            let last_idx = map.entry(ch).or_insert(idx);
            *last_idx = idx;
            vec.push(ch);
        }

        let mut res: Vec<i32> = Vec::new();
        let mut ptr = 0;
        let mut size = 1;
        let mut end = *map.get(&vec[ptr]).unwrap();

        while ptr < s.len() {
            size = 1;
            end = *map.get(&vec[ptr]).unwrap();
            while ptr < end {
                end = max(end, *map.get(&vec[ptr]).unwrap());
                ptr += 1;
                size += 1;
            }
            res.push(size);
            ptr += 1;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_763() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
        assert_eq!(
            Solution::partition_labels("eccbbbbdec".to_string()),
            vec![10]
        );
        assert_eq!(
            Solution::partition_labels("aabaccbdeffed".to_string()),
            vec![7, 6]
        );
    }
}
