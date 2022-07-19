/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 * Example 2:
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 * Example 3:
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *
 * Constraints:
 *
 * 	The number of nodes in each linked list is in the range [1, 100].
 * 	0 <= Node.val <= 9
 * 	It is guaranteed that the list represents a number that does not have leading zeros.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry: i32 = 0;
        let mut res_head = Some(Box::new(ListNode::new(0)));
        let empty_node = Some(Box::new(ListNode::new(0)));
        let mut moving_head = &mut res_head;

        while l1.as_ref() != None || l2.as_ref() != None {
            if l1.as_ref() == None {
                l1 = empty_node.clone();
            }
            if l2.as_ref() == None {
                l2 = empty_node.clone();
            }
            let mut next_node = Some(Box::new(ListNode::new(0)));
            (&mut next_node).as_mut().unwrap().val =
                l1.as_ref().unwrap().val + l2.as_ref().unwrap().val + carry;
            carry = 0;
            if next_node.as_ref().unwrap().val > 9 {
                (&mut next_node).as_mut().unwrap().val -= 10;
                carry = 1;
            }
            moving_head.as_mut().unwrap().next = next_node;
            moving_head = &mut moving_head.as_mut().unwrap().next;
            l1 = l1.unwrap().next;
            l2 = l2.unwrap().next;
        }
        if carry == 1 {
            let next_node = Some(Box::new(ListNode::new(1)));
            moving_head.as_mut().unwrap().next = next_node;
        }
        res_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        );
        assert_eq!(
            Solution::add_two_numbers(
                to_list(vec![9, 9, 9, 9, 9, 9, 9]),
                to_list(vec![9, 9, 9, 9])
            ),
            to_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
