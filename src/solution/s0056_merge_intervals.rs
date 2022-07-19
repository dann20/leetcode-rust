/**
 * [56] Merge Intervals
 *
 * Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
 *
 * Example 1:
 *
 * Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 *
 * Example 2:
 *
 * Input: intervals = [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 *
 *
 * Constraints:
 *
 * 	1 <= intervals.length <= 10^4
 * 	intervals[i].length == 2
 * 	0 <= starti <= endi <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-intervals/
// discuss: https://leetcode.com/problems/merge-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::max;

        let mut intervals = intervals;
        intervals.sort_by_key(|k| k[0]);

        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(intervals[0].clone());

        for interval in intervals[1..].iter() {
            let (start, end) = (interval[0], interval[1]);
            let last_interval = res.last().unwrap().clone();
            let (last_start, last_end) = (last_interval[0], last_interval[1]);
            if start <= last_end {
                let new_end = max(last_end, end);
                res.pop();
                res.push(vec![last_start, new_end]);
            } else {
                res.push(vec![start, end]);
            }
        }
        res
    }

    pub fn my_merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn combine(one: &Vec<i32>, two: &Vec<i32>) -> Option<Vec<i32>> {
            use std::cmp::max;
            if one[0] <= two[0] && two[0] <= one[1] {
                Some(vec![one[0], max(one[1], two[1])])
            } else if two[0] <= one[0] && one[0] <= two[1] {
                Some(vec![two[0], max(one[1], two[1])])
            } else {
                None
            }
        }

        let mut res: Vec<Vec<i32>> = intervals;
        let mut len = res.len();
        let mut ptr = 0;
        let mut inc = true;
        while ptr < len {
            inc = true;
            for i in (ptr + 1)..len {
                if let Some(vec) = combine(&res[ptr], &res[i]) {
                    res.remove(i);
                    res.remove(ptr);
                    res.insert(ptr, vec);
                    len = res.len();
                    inc = false;
                    break;
                }
            }
            if inc {
                ptr += 1;
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_56() {
        let input1 = [[1, 3], [2, 6], [8, 10], [15, 18]];
        let ans1 = [[1, 6], [8, 10], [15, 18]];
        assert_eq!(
            Solution::merge(
                input1
                    .to_vec()
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<Vec<i32>>>()
            ),
            ans1.to_vec()
                .iter()
                .map(|x| x.to_vec())
                .collect::<Vec<Vec<i32>>>(),
        );

        let input2 = [[1, 4], [4, 5]];
        let ans2 = [[1, 5]];
        assert_eq!(
            Solution::merge(
                input2
                    .to_vec()
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<Vec<i32>>>()
            ),
            ans2.to_vec()
                .iter()
                .map(|x| x.to_vec())
                .collect::<Vec<Vec<i32>>>(),
        );
    }
}
