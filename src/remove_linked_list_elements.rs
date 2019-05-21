use crate::leetcode::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut s_result = &mut result;
        let mut head = head.as_ref();
        while head.is_some() {
            if head.unwrap().val != val {
                s_result.next = Some(Box::new(ListNode::new(head.unwrap().val)));
                s_result = s_result.next.as_mut().unwrap();
            }
            head = head.unwrap().next.as_ref();
        }
        result.next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn remove_elements() {
        assert_eq!(
            linkedlist![1, 2, 3, 4, 5],
            Solution::remove_elements(linkedlist![1, 2, 6, 3, 4, 5, 6], 6)
        );

        assert_eq!(
            linkedlist![1, 2, 3, 4, 5],
            Solution::remove_elements(linkedlist![6, 1, 2, 6, 3, 4, 5, 6], 6)
        );

        assert_eq!(
            linkedlist![],
            Solution::remove_elements(linkedlist![6, 6], 6)
        );
    }
}
