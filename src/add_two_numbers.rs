use crate::leetcode::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: Box<ListNode> = Box::new(ListNode::new(0));
        let mut s_result = &mut result;
        let (mut l1, mut l2) = (l1, l2);
        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let x = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => 0,
            };

            let y = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => 0,
            };
            let sum = x + y + carry;
            carry = if sum > 9 { 1 } else { 0 };
            s_result.next = Some(Box::new(ListNode::new(sum % 10)));
            s_result = s_result.next.as_mut().unwrap();
        }
        if carry > 0 {
            s_result.next = Some(Box::new(ListNode::new(1)));
        }
        result.next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn add_two_numbers() {
        assert_eq!(
            linkedlist![6, 7, 8],
            Solution::add_two_numbers(linkedlist![3, 2, 1], linkedlist![3, 5, 7])
        );

        assert_eq!(
            linkedlist![0],
            Solution::add_two_numbers(linkedlist![0], linkedlist![0])
        );

        assert_eq!(
            linkedlist![0, 1, 1],
            Solution::add_two_numbers(linkedlist![1, 1], linkedlist![9, 9])
        );
    }
}
