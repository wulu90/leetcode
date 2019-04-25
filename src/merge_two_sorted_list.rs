use crate::linked_list;
use crate::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        linked_list::vec_to_linklist(vec![1, 2, 4])
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn merge_two_list() {}
}
