/**
 * [867] Transpose Matrix
 *
 * Given a 2D integer array matrix, return the transpose of matrix.
 * The transpose of a matrix is the matrix flipped over its main diagonal, switching the matrix's row and column indices.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/10/hint_transpose.png" style="width: 600px; height: 197px;" />
 *
 * Example 1:
 *
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [[1,4,7],[2,5,8],[3,6,9]]
 *
 * Example 2:
 *
 * Input: matrix = [[1,2,3],[4,5,6]]
 * Output: [[1,4],[2,5],[3,6]]
 *
 *
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 1000
 * 	1 <= m * n <= 10^5
 * 	-10^9 <= matrix[i][j] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/transpose-matrix/
// discuss: https://leetcode.com/problems/transpose-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut transposed = vec![vec![0; m]; n];
        for i in 0..m {
            for j in 0..n {
                transposed[j][i] = matrix[i][j];
            }
        }
        transposed
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_867() {
        let input1 = vec2d![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let ans1 = vec2d![[1, 4, 7], [2, 5, 8], [3, 6, 9]];
        assert_eq!(Solution::transpose(input1), ans1);

        let input2 = vec2d![[1, 2, 3], [4, 5, 6]];
        let ans2 = vec2d![[1, 4], [2, 5], [3, 6]];
        assert_eq!(Solution::transpose(input2), ans2);
    }
}
