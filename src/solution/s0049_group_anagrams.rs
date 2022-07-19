/**
 * [49] Group Anagrams
 *
 * Given an array of strings strs, group the anagrams together. You can return the answer in any order.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *
 * Example 1:
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * Example 2:
 * Input: strs = [""]
 * Output: [[""]]
 * Example 3:
 * Input: strs = ["a"]
 * Output: [["a"]]
 *
 * Constraints:
 *
 * 	1 <= strs.length <= 10^4
 * 	0 <= strs[i].length <= 100
 * 	strs[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/group-anagrams/
// discuss: https://leetcode.com/problems/group-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut sorted = strs.clone();
        for s in sorted.iter_mut() {
            let mut l: Vec<char> = s.chars().collect();
            l.sort();
            *s = l.into_iter().collect();
        }

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for (idx, s) in sorted.iter().enumerate() {
            let group = map.entry(s.to_string()).or_insert(vec![]);
            group.push(strs[idx].clone());
        }
        map.into_values().collect::<Vec<Vec<String>>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    fn test_case(input: Vec<&str>, ans: Vec<Vec<&str>>) {
        let output =
            Solution::group_anagrams(input.iter().map(|x| x.to_string()).collect::<Vec<String>>());
        println!("{:?}", output);
        let ans = ans
            .iter()
            .map(|x| {
                let mut group: Vec<String> = Vec::new();
                for val in x.into_iter() {
                    group.push(val.to_string());
                }
                group
            })
            .collect::<Vec<Vec<String>>>();
        println!("{:?}", ans);

        assert_eq!(output.len(), ans.len());
        for group in ans.iter() {
            assert_eq!(output.contains(group), true);
            println!("{:?}", group);
        }
    }

    #[test]
    fn test_49() {
        // strings in each group appear in the order they appear in input.
        let input1 = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let ans1 = vec2d![["bat"], ["tan", "nat"], ["eat", "tea", "ate"]];

        let input2 = vec![""];
        let ans2 = vec2d![[""]];

        let input3 = vec!["a"];
        let ans3 = vec2d![["a"]];

        test_case(input1, ans1);
        test_case(input2, ans2);
        test_case(input3, ans3);
    }
}
