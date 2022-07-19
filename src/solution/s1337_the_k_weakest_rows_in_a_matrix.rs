/**
 * [1337] The K Weakest Rows in a Matrix
 *
 * You are given an m x n binary matrix mat of 1's (representing soldiers) and 0's (representing civilians). The soldiers are positioned in front of the civilians. That is, all the 1's will appear to the left of all the 0's in each row.
 * A row i is weaker than a row j if one of the following is true:
 *
 * 	The number of soldiers in row i is less than the number of soldiers in row j.
 * 	Both rows have the same number of soldiers and i < j.
 *
 * Return the indices of the k weakest rows in the matrix ordered from weakest to strongest.
 *
 * Example 1:
 *
 * Input: mat =
 * [[1,1,0,0,0],
 *  [1,1,1,1,0],
 *  [1,0,0,0,0],
 *  [1,1,0,0,0],
 *  [1,1,1,1,1]],
 * k = 3
 * Output: [2,0,3]
 * Explanation:
 * The number of soldiers in each row is:
 * - Row 0: 2
 * - Row 1: 4
 * - Row 2: 1
 * - Row 3: 2
 * - Row 4: 5
 * The rows ordered from weakest to strongest are [2,0,3,1,4].
 *
 * Example 2:
 *
 * Input: mat =
 * [[1,0,0,0],
 *  [1,1,1,1],
 *  [1,0,0,0],
 *  [1,0,0,0]],
 * k = 2
 * Output: [0,2]
 * Explanation:
 * The number of soldiers in each row is:
 * - Row 0: 1
 * - Row 1: 4
 * - Row 2: 1
 * - Row 3: 1
 * The rows ordered from weakest to strongest are [0,2,3,1].
 *
 *
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	2 <= n, m <= 100
 * 	1 <= k <= m
 * 	matrix[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
// discuss: https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let [m, n] = [mat.len(), mat[0].len()];
        let mut count_vec: Vec<(usize, i32)> = Vec::new();
        for row in 0..m {
            // let mut count: i32 = 0;
            // for col in 0..n {
            //     count += mat[row][col];
            // }
            let count: i32 = mat[row].iter().sum();
            count_vec.push((row, count));
        }
        for i in 0..count_vec.len() {
            let mut swapped: bool = false;
            for j in 0..(count_vec.len() - i - 1) {
                if count_vec[j].1 > count_vec[j + 1].1 {
                    count_vec.swap(j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
        let mut result: Vec<i32> = Vec::new();
        for idx in 0..(k as usize) {
            result.push(count_vec[idx].0 as i32);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1337() {
        let arr1 = [
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ];
        let result1 = [2, 0, 3];
        assert_eq!(
            Solution::k_weakest_rows(
                arr1.to_vec()
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<Vec<i32>>>(),
                3
            ),
            Vec::from(result1)
        );
        println!("{:#?}", arr1);

        let arr2 = [[1, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0]];
        let result2 = [0, 2];
        assert_eq!(
            Solution::k_weakest_rows(
                arr2.to_vec()
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<Vec<i32>>>(),
                2
            ),
            Vec::from(result2)
        );
        println!("{:#?}", arr2);

        let arr3 = vec2d![[1, 1, 1], [0, 1, 0], [0, 0, 1]];
        assert_eq!(Solution::k_weakest_rows(arr3, 1), vec![1]);
    }
}
