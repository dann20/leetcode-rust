/**
 * [25] Reverse Nodes in k-Group
 *
 * Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
 * k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
 * You may not alter the values in the list's nodes, only nodes themselves may be changed.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], k = 2
 * Output: [2,1,4,3,5]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], k = 3
 * Output: [3,2,1,4,5]
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the list is n.
 * 	1 <= k <= n <= 5000
 * 	0 <= Node.val <= 1000
 *
 *
 * Follow-up: Can you solve the problem in O(1) extra memory space?
 *
 */
pub struct Solution {}
use std::ops::Deref;

use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/reverse-nodes-in-k-group/
// discuss: https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn check_enough_k(head: Option<Box<ListNode>>, k: i32) -> bool {
            if head.is_none() {
                return false;
            }
            let mut head = head.clone();
            let mut i = 0;
            while head.is_some() {
                i += 1;
                head = head.unwrap().next.take();
                if i == k {
                    return true;
                }
            }
            false
        }

        fn reverse(
            head: &mut Option<Box<ListNode>>,
            k: i32,
            prev: Option<Box<ListNode>>,
            prev_group: Option<Box<ListNode>>,
        ) {
            use std::mem::swap;
            if head.is_none() {
                return;
            }
            let mut prev_group = prev_group;
            if check_enough_k(head.clone(), k) {
                let mut head = head;
                let mut prev = prev;
                let mut prev_group_new = head.clone();
                let mut next_node: Option<Box<ListNode>>;
                for _ in 0..k {
                    // next_node = head.as_mut().unwrap().next.take();
                    // head.as_mut().unwrap().next = prev;
                    swap(&mut head.as_mut().unwrap().next, &mut prev);
                    // prev = head.take();
                    swap(&mut prev, head);
                    // swap(head, &mut next_node);
                }
                if let Some(mut pg) = prev_group {
                    pg.next = prev.clone();
                }
                reverse(head, k, None, prev_group_new);
            } else {
                if let Some(mut pg) = prev_group {
                    pg.next = head.clone();
                }
            }
        }

        let mut res = head;
        let mut head = &mut res;
        let mut prev: Option<Box<ListNode>> = None;
        let mut prev_group: Option<Box<ListNode>> = None;

        reverse(head, k, prev, prev_group);

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5, 6]), 2),
            to_list(vec![2, 1, 4, 3, 6, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5, 6]), 3),
            to_list(vec![3, 2, 1, 6, 5, 4])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3),
            to_list(vec![3, 2, 1, 4, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1]), 1),
            to_list(vec![1])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1]), 2),
            to_list(vec![1])
        );
    }
}
