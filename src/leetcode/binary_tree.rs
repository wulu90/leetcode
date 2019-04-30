//use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    let root = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    queue.push_back(root.as_ref().unwrap().clone());
    //queue.push_back(Rc::clone(root.as_ref().unwrap()));
    for a in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = a[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if a.len() > 1 {
            if let Some(v) = a[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    root
}

#[macro_export]
macro_rules! binarytree {
    () => {
        None
    };

    ($($s:expr),*) => {{
        let vec=vec![$(stringify!($s)),*];
        let vec=vec.into_iter().map(|x| x.parse::<i32>().ok()).collect::<Vec<_>>();
        crate::leetcode::binary_tree::to_tree(vec)}
    };
}
