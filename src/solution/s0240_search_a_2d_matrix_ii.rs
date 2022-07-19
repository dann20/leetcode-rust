/**
 * [240] Search a 2D Matrix II
 *
 * Write an efficient algorithm that searches for a value target in an m x n integer matrix matrix. This matrix has the following properties:
 *
 * 	Integers in each row are sorted in ascending from left to right.
 * 	Integers in each column are sorted in ascending from top to bottom.
 *
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/24/searchgrid2.jpg" style="width: 300px; height: 300px;" />
 * Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/24/searchgrid.jpg" style="width: 300px; height: 300px;" />
 * Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
 * Output: false
 *
 *
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= n, m <= 300
 * 	-10^9 <= matrix[i][j] <= 10^9
 * 	All the integers in each row are sorted in ascending order.
 * 	All the integers in each column are sorted in ascending order.
 * 	-10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-a-2d-matrix-ii/
// discuss: https://leetcode.com/problems/search-a-2d-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;

        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;

        let mut col = n - 1;
        let mut row = 0;
        while row < m && col >= 0 {
            match matrix[row as usize][col as usize].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater => col -= 1,
                Ordering::Less => row += 1,
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_240() {
        let input1 = vec2d![
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30]
        ];
        assert_eq!(Solution::search_matrix(input1, 5), true);

        let input2 = vec2d![
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30]
        ];
        assert_eq!(Solution::search_matrix(input2, 20), false);
    }
}
