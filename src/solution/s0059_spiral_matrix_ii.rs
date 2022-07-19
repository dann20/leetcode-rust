/**
 * [59] Spiral Matrix II
 *
 * Given a positive integer n, generate an n x n matrix filled with elements from 1 to n^2 in spiral order.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiraln.jpg" style="width: 242px; height: 242px;" />
 * Input: n = 3
 * Output: [[1,2,3],[8,9,4],[7,6,5]]
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: [[1]]
 *
 *
 * Constraints:
 *
 * 	1 <= n <= 20
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix-ii/
// discuss: https://leetcode.com/problems/spiral-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 1 {
            return vec![vec![1]];
        }

        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut last = 0i32;
        let mut l = 0usize;
        let mut r = (n - 1) as usize;
        while l < r {
            let (t, b) = (l.clone(), r.clone());
            for i in l..r {
                matrix[t][i] = last + 1;
                last += 1;
            }
            for i in t..b {
                matrix[i][r] = last + 1;
                last += 1;
            }
            for i in ((l + 1)..(r + 1)).rev() {
                matrix[b][i] = last + 1;
                last += 1;
            }
            for i in ((t + 1)..(b + 1)).rev() {
                matrix[i][l] = last + 1;
                last += 1;
            }
            l += 1;
            r -= 1;
        }
        if l == r {
            matrix[l][l] = last + 1;
        }
        matrix
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_59() {
        let ans1 = vec2d![[1, 2, 3], [8, 9, 4], [7, 6, 5]];
        assert_eq!(Solution::generate_matrix(3), ans1);

        let ans2 = vec2d![[1]];
        assert_eq!(Solution::generate_matrix(1), ans2);

        let ans3 = vec2d![
            [1, 2, 3, 4],
            [12, 13, 14, 5],
            [11, 16, 15, 6],
            [10, 9, 8, 7]
        ];
        assert_eq!(Solution::generate_matrix(4), ans3);
    }
}
