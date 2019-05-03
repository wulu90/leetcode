use super::leetcode::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = None;
        let mut head = &head;
        while head.is_some() {
            let mut node = ListNode::new(head.as_ref().unwrap().val);
            node.next = current;
            current = Some(Box::new(node));
            head = &(head.as_ref().unwrap().next);
        }

        current
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn reverse_list() {
        assert_eq!(
            linkedlist![5, 4, 3, 2, 1],
            Solution::reverse_list(linkedlist![1, 2, 3, 4, 5])
        );
    }
}
