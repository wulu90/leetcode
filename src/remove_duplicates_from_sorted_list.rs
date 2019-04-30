use crate::leetcode::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = head.as_ref();
        let mut pre = head.unwrap().val;
        let mut result = Box::new(ListNode::new(0));
        let mut s_result = &mut result;
        s_result.next = Some(Box::new(ListNode::new(pre)));
        s_result = s_result.next.as_mut().unwrap();
        while let Some(node) = head {
            if node.val != pre {
                s_result.next = Some(Box::new(ListNode::new(node.val)));
                s_result = s_result.next.as_mut().unwrap();
                pre = node.val;
            }
            head = node.next.as_ref();
        }
        result.next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn delete_duplicates() {
        assert_eq!(
            linkedlist![1, 2],
            Solution::delete_duplicates(linkedlist![1, 1, 2])
        );

        assert_eq!(linkedlist![], Solution::delete_duplicates(linkedlist![]));

        assert_eq!(linkedlist![1], Solution::delete_duplicates(linkedlist![1]));

        assert_eq!(
            linkedlist![1, 2, 3, 4],
            Solution::delete_duplicates(linkedlist![1, 2, 3, 4])
        );

        assert_eq!(
            linkedlist![1, 2, 3, 4],
            Solution::delete_duplicates(linkedlist![1, 1, 2, 2, 2, 3, 3, 3, 4, 4])
        );
    }
}
