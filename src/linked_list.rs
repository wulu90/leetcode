#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
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

#[macro_export]
macro_rules! linkedlist {
    ($($x:expr),*) => {
        crate::linked_list::vec_to_linklist(vec![$($x.to_owned()), *])
    }
}
