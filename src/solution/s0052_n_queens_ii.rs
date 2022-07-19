/**
 * [52] N-Queens II
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
 * Input: n = 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 *
 *
 * Constraints:
 *
 * 	1 <= n <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens-ii/
// discuss: https://leetcode.com/problems/n-queens-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        use std::collections::HashSet;

        let mut res: i32 = 0;
        let mut col_set: HashSet<i32> = HashSet::new();
        let mut pos_diag_set: HashSet<i32> = HashSet::new(); // r + c
        let mut neg_diag_set: HashSet<i32> = HashSet::new(); // r - c

        fn backtrack(
            r: i32,
            n: i32,
            cols: &mut HashSet<i32>,
            pos_diags: &mut HashSet<i32>,
            neg_diags: &mut HashSet<i32>,
            res: &mut i32,
        ) {
            if r == n {
                *res += 1;
                return;
            }
            for c in 0..n {
                if cols.contains(&c) || pos_diags.contains(&(r + c)) || neg_diags.contains(&(r - c))
                {
                    continue;
                }
                cols.insert(c);
                pos_diags.insert(r + c);
                neg_diags.insert(r - c);

                backtrack(r + 1, n, cols, pos_diags, neg_diags, res);

                cols.remove(&c);
                pos_diags.remove(&(r + c));
                neg_diags.remove(&(r - c));
            }
        }

        backtrack(
            0,
            n,
            &mut col_set,
            &mut pos_diag_set,
            &mut neg_diag_set,
            &mut res,
        );
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_52() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
