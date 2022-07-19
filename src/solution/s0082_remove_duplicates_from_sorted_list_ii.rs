use crate::util::linked_list::{to_list, ListNode};

/**
 * [82] Remove Duplicates from Sorted List II
 *
 * Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/linkedlist1.jpg" style="width: 500px; height: 142px;" />
 * Input: head = [1,2,3,3,4,4,5]
 * Output: [1,2,5]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/linkedlist2.jpg" style="width: 500px; height: 205px;" />
 * Input: head = [1,1,1,2,3]
 * Output: [2,3]
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 300].
 * 	-100 <= Node.val <= 100
 * 	The list is guaranteed to be sorted in ascending order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref().is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let empty = Some(Box::new(ListNode::new(-200)));
        let mut res = Some(Box::new(ListNode::new(0)));
        res.as_mut().unwrap().next = head.clone();
        let mut before = &mut res;

        let mut ptr1 = &head;
        let mut ptr2 = &(head.as_ref().unwrap().next);
        let mut temp: Option<i32> = Some(200);

        while ptr1.as_ref().is_some() {
            let n1 = ptr1.as_ref().unwrap();
            let n2 = ptr2.as_ref().unwrap();
            ptr1 = &(n1.next);
            if n1.val == n2.val || n1.val == temp.unwrap() {
                temp = Some(n1.val);
                before.as_mut().unwrap().next = ptr1.clone();
            } else {
                before = &mut before.as_mut().unwrap().next;
            }
            ptr2 = if n2.next.is_some() { &(n2.next) } else { &empty };
        }

        res.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_82() {
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 2, 3, 3, 4, 4, 5])),
            to_list(vec![1, 2, 5])
        );
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 1, 2, 3])),
            to_list(vec![2, 3])
        );
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1])),
            to_list(vec![])
        );
    }
}
