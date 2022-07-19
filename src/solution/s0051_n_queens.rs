/**
 * [51] N-Queens
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 * Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
 * Input: n = 4
 * Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: [["Q"]]
 *
 *
 * Constraints:
 *
 * 	1 <= n <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens/
// discuss: https://leetcode.com/problems/n-queens/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        use std::collections::HashSet;

        let mut res: Vec<Vec<String>> = Vec::new();
        let mut board: Vec<Vec<String>> = vec![vec![".".to_string(); n as usize]; n as usize];
        let mut col_set: HashSet<i32> = HashSet::new();
        let mut pos_diag_set: HashSet<i32> = HashSet::new(); // r + c
        let mut neg_diag_set: HashSet<i32> = HashSet::new(); // r - c

        fn backtrack(
            r: i32,
            n: i32,
            cols: &mut HashSet<i32>,
            pos_diags: &mut HashSet<i32>,
            neg_diags: &mut HashSet<i32>,
            board: &mut Vec<Vec<String>>,
            res: &mut Vec<Vec<String>>,
        ) {
            if r == n {
                let mut copy: Vec<String> = vec![".".to_string(); n as usize];
                for (idx, row) in board.iter().enumerate() {
                    copy[idx] = row.join("");
                }
                res.push(copy);
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
                board[r as usize][c as usize] = "Q".to_string();

                backtrack(r + 1, n, cols, pos_diags, neg_diags, board, res);

                cols.remove(&c);
                pos_diags.remove(&(r + c));
                neg_diags.remove(&(r - c));
                board[r as usize][c as usize] = ".".to_string();
            }
        }

        backtrack(
            0,
            n,
            &mut col_set,
            &mut pos_diag_set,
            &mut neg_diag_set,
            &mut board,
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
    fn test_51() {
        let ans1 = vec![
            [".Q..".to_string(), "...Q".to_string(), "Q...".to_string(), "..Q.".to_string()],
            ["..Q.".to_string(), "Q...".to_string(), "...Q".to_string(), ".Q..".to_string()],
        ];
        assert_eq!(Solution::solve_n_queens(4), ans1.into_iter().map(|x| x.to_vec()).collect::<Vec<Vec<String>>>());

        let ans2 = vec![["Q".to_string()]];
        assert_eq!(Solution::solve_n_queens(1), ans2.into_iter().map(|x| x.to_vec()).collect::<Vec<Vec<String>>>());
    }
}
