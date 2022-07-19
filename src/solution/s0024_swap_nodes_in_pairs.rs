/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [2,1,4,3]
 *
 * Example 2:
 *
 * Input: head = []
 * Output: []
 *
 * Example 3:
 *
 * Input: head = [1]
 * Output: [1]
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 100].
 * 	0 <= Node.val <= 100
 *
 */
pub struct Solution {}
use std::borrow::BorrowMut;

use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/swap-nodes-in-pairs/
// discuss: https://leetcode.com/problems/swap-nodes-in-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref().is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut res = head;
        let mut ptr = &mut res;

        while ptr.as_ref().is_some() && ptr.as_ref().unwrap().next.is_some() {
            let mut next_node = ptr.as_mut().unwrap().next.take();
            let next_next_node = next_node.as_mut().unwrap().next.take();
            std::mem::swap(ptr, &mut next_node);
            next_node.as_mut().unwrap().next = next_next_node;
            ptr.as_mut().unwrap().next = next_node;
            ptr = &mut ptr.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4, 5, 6])),
            to_list(vec![2, 1, 4, 3, 6, 5])
        );
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])),
            to_list(vec![2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 11])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
    }
}
