pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn vec_to_linklist(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current

    /*
    let mut node = Some(Box::new(ListNode::new(0)));
    let mut shadow_node = &mut node;

    for i in 0..vec.len() {
        shadow_node.as_mut().unwrap().val = vec[i];
        if i != (vec.len() - 1) {
            shadow_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(0)));;
            shadow_node = &mut shadow_node.as_mut().unwrap().next;
        }
    }
    node
    */
}

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
    use crate::add_two_numbers::vec_to_linklist;

    #[test]
    fn add_two_numbers() {
        assert_eq!(
            vec_to_linklist(vec![6, 7, 8]),
            Solution::add_two_numbers(
                vec_to_linklist(vec![3, 2, 1]),
                vec_to_linklist(vec![3, 5, 7])
            )
        );

        assert_eq!(
            vec_to_linklist(vec![0]),
            Solution::add_two_numbers(vec_to_linklist(vec![0]), vec_to_linklist(vec![0]))
        );

        assert_eq!(
            vec_to_linklist(vec![0, 1, 1]),
            Solution::add_two_numbers(vec_to_linklist(vec![1, 1]), vec_to_linklist(vec![9, 9]))
        );
    }
}
