/**
 * [187] Repeated DNA Sequences
 *
 * The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
 *
 * 	For example, "ACGAATTCCG" is a DNA sequence.
 *
 * When studying DNA, it is useful to identify repeated sequences within the DNA.
 * Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.
 *
 * Example 1:
 * Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
 * Output: ["AAAAACCCCC","CCCCCAAAAA"]
 * Example 2:
 * Input: s = "AAAAAAAAAAAAA"
 * Output: ["AAAAAAAAAA"]
 *
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either 'A', 'C', 'G', or 'T'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/repeated-dna-sequences/
// discuss: https://leetcode.com/problems/repeated-dna-sequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashSet;

        if s.len() <= 10 {
            return vec![];
        }

        let mut seen: HashSet<String> = HashSet::new();
        let mut repeated: HashSet<String> = HashSet::new();

        for i in 0..(s.len() - 9) {
            if !seen.insert(s[i..(i + 10)].to_string()) {
                repeated.insert(s[i..(i + 10)].to_string());
            }
        }
        repeated.into_iter().collect::<Vec<String>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_187() {
        let input1 = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
        let ans1 = vec!["AAAAACCCCC", "CCCCCAAAAA"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        test_case(input1, ans1);

        let input2 = "AAAAAAAAAAAAA".to_string();
        let ans2 = vec!["AAAAAAAAAA"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        test_case(input2, ans2);

        let input3 = "CCCCCCCC".to_string();
        let ans3 = vec![];
        test_case(input3, ans3);

        let input4 = "CCCCCCCCCC".to_string();
        let ans4 = vec![];
        test_case(input4, ans4);
    }

    fn test_case(input: String, ans: Vec<String>) {
        let output = Solution::find_repeated_dna_sequences(input);
        assert_eq!(output.len(), ans.len());
        for s in output.iter() {
            assert_eq!(ans.contains(&s), true);
        }
    }
}
