/**
 * [6] Zigzag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 * string convert(string s, int numRows);
 *
 *
 * Example 1:
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 * Example 2:
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 * Example 3:
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 	1 <= numRows <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/
// discuss: https://leetcode.com/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut matrix: Vec<Vec<Option<char>>> = Vec::new();
        // let s_vec = s.chars().collect::<Vec<char>>();
        let mut char_iter = s.chars();

        matrix.push(Vec::new());
        for i in 0..num_rows {
            matrix.last_mut().unwrap().push(char_iter.next());
        }
        matrix.push(Vec::new());
        // matrix.last_mut().unwrap().push

        let num_cols = num_rows;
        let mut left: i32 = s.len() as i32;
        let mut num_rows = 0;
        let mut full = true;
        while left > 0 {
            if full {
                num_rows += 1;
                left -= num_cols;
            } else {}
        }

        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
    }
}
