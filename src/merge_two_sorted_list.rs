use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut s_result = &mut result;
        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
        while l1.is_some() || l2.is_some() {
            match (l1, l2) {
                (Some(l1node), Some(l2node)) => {
                    if l1node.val >= l2node.val {
                        s_result.next = Some(Box::new(ListNode::new(l2node.val)));
                        l2 = l2node.next.as_ref();
                    } else {
                        s_result.next = Some(Box::new(ListNode::new(l1node.val)));
                        l1 = l1node.next.as_ref();
                    }
                }
                (Some(l1node), None) => {
                    s_result.next = Some(Box::new(ListNode::new(l1node.val)));
                    l1 = l1node.next.as_ref();
                }
                (None, Some(l2node)) => {
                    s_result.next = Some(Box::new(ListNode::new(l2node.val)));
                    l2 = l2node.next.as_ref();
                }
                _ => {}
            }
            s_result = s_result.next.as_mut().unwrap();
        }
        result.next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn merge_two_list() {
        assert_eq!(
            linkedlist![1, 2, 3, 4],
            Solution::merge_two_lists(linkedlist![1, 2], linkedlist![3, 4])
        );
        assert_eq!(
            linkedlist![1, 1, 2, 3, 4, 4],
            Solution::merge_two_lists(linkedlist![1, 2, 4], linkedlist![1, 3, 4])
        );
        assert_eq!(
            linkedlist![1, 2],
            Solution::merge_two_lists(linkedlist![1, 2], linkedlist![])
        );
        assert_eq!(
            linkedlist![3, 4],
            Solution::merge_two_lists(linkedlist![], linkedlist![3, 4])
        );
        assert_eq!(
            linkedlist![1, 3, 6],
            Solution::merge_two_lists(linkedlist![1, 6], linkedlist![3])
        );
        assert_eq!(
            linkedlist![1, 2, 4],
            Solution::merge_two_lists(linkedlist![2], linkedlist![1, 4])
        );
        assert_eq!(
            linkedlist![-1, -2, 3, 4],
            Solution::merge_two_lists(linkedlist![-1, -2], linkedlist![3, 4])
        );
    }
}
