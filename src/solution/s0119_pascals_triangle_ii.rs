/**
 * [119] Pascal's Triangle II
 *
 * Given an integer rowIndex, return the rowIndex^th (0-indexed) row of the Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" />
 *
 * Example 1:
 * Input: rowIndex = 3
 * Output: [1,3,3,1]
 * Example 2:
 * Input: rowIndex = 0
 * Output: [1]
 * Example 3:
 * Input: rowIndex = 1
 * Output: [1,1]
 *
 * Constraints:
 *
 * 	0 <= rowIndex <= 33
 *
 *
 * Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pascals-triangle-ii/
// discuss: https://leetcode.com/problems/pascals-triangle-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        match row_index {
            0 => return vec![1],
            1 => return vec![1, 1],
            _ => (),
        };
        let prev_row = Solution::get_row(row_index - 1);
        let mut row = vec![1];
        for idx in 1..row_index as usize {
            row.push(prev_row[idx] + prev_row[idx - 1]);
        }
        row.push(1);
        row
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_119() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(5), vec![1, 5, 10, 10, 5, 1]);
    }
}
