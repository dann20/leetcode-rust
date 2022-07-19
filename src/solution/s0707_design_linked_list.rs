use surf::middleware::logger::new;

/**
 * [707] Design Linked List
 *
 * Design your implementation of the linked list. You can choose to use a singly or doubly linked list.<br />
 * A node in a singly linked list should have two attributes: val and next. val is the value of the current node, and next is a pointer/reference to the next node.<br />
 * If you want to use the doubly linked list, you will need one more attribute prev to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.
 * Implement the MyLinkedList class:
 *
 * 	MyLinkedList() Initializes the MyLinkedList object.
 * 	int get(int index) Get the value of the index^th node in the linked list. If the index is invalid, return -1.
 * 	void addAtHead(int val) Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
 * 	void addAtTail(int val) Append a node of value val as the last element of the linked list.
 * 	void addAtIndex(int index, int val) Add a node of value val before the index^th node in the linked list. If index equals the length of the linked list, the node will be appended to the end of the linked list. If index is greater than the length, the node will not be inserted.
 * 	void deleteAtIndex(int index) Delete the index^th node in the linked list, if the index is valid.
 *
 *
 * Example 1:
 *
 * Input
 * ["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
 * [[], [1], [3], [1, 2], [1], [1], [1]]
 * Output
 * [null, null, null, null, 2, null, 3]
 * Explanation
 * MyLinkedList myLinkedList = new MyLinkedList();
 * myLinkedList.addAtHead(1);
 * myLinkedList.addAtTail(3);
 * myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
 * myLinkedList.get(1);              // return 2
 * myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
 * myLinkedList.get(1);              // return 3
 *
 *
 * Constraints:
 *
 * 	0 <= index, val <= 1000
 * 	Please do not use the built-in LinkedList library.
 * 	At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and deleteAtIndex.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-linked-list/
// discuss: https://leetcode.com/problems/design-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
#[derive(Debug)]
struct MyLinkedList {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        let mut i = 0;
        let mut res = self.head.as_deref();
        while i < index {
            i += 1;
            res = res.unwrap().next.as_deref();
            if res.is_none() {
                return -1;
            }
        }
        if let Some(node) = res {
            node.val
        } else {
            -1
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut node = self.head.as_deref_mut();
        let new_node = Box::new(Node { val, next: None });
        if node.is_none() {
            self.head = Some(new_node);
            return;
        }
        while node.as_ref().unwrap().next.is_some() {
            node = node.unwrap().next.as_deref_mut();
        }
        node.unwrap().next = Some(new_node);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 {
            return;
        } else if index == 0 {
            self.add_at_head(val);
            return;
        }
        let mut i = 0;
        let mut node = self.head.as_deref_mut();
        while i < index - 1 {
            if node.as_ref().is_none() {
                return;
            }
            i += 1;
            node = node.unwrap().next.as_deref_mut();
        }
        if node.as_ref().is_some() {
            let next_node = node.as_mut().unwrap().next.take();
            let new_node = Box::new(Node {
                val,
                next: next_node,
            });
            node.unwrap().next = Some(new_node);
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        } else if index == 0 {
            let next_node = self.head.as_mut().unwrap().next.take();
            self.head = next_node;
            return;
        }
        let mut i = 0;
        let mut node = self.head.as_deref_mut();
        while i < index - 1 {
            if node.as_ref().unwrap().next.is_none() {
                return;
            }
            i += 1;
            node = node.unwrap().next.as_deref_mut();
        }
        let mut next_node = node.as_mut().unwrap().next.take();
        let mut next_next_node = if next_node.as_ref().is_some() {
            next_node.as_mut().unwrap().next.take()
        } else {
            None
        };
        node.unwrap().next = next_next_node;
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_707() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(2);
        list.add_at_head(3);
        list.add_at_head(4);
        list.delete_at_index(0);
        list.add_at_index(0, 100);
        list.add_at_index(0, 500);
        list.add_at_index(10, 99);
        dbg!(list);
    }
}
