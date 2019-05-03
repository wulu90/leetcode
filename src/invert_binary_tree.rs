use crate::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root.clone() {
            Some(node) => {
                let invert_left = Self::invert_tree(node.borrow().left.clone());
                let invert_right = Self::invert_tree(node.borrow().right.clone());
                node.borrow_mut().left = invert_right;
                node.borrow_mut().right = invert_left;
            }
            None => {}
        }
        root
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn invert_tree() {
        assert_eq!(
            binarytree![4, 7, 2, 9, 6, 3, 1],
            Solution::invert_tree(binarytree![4, 2, 7, 1, 3, 6, 9])
        );

        assert_eq!(
            binarytree![4, 1, 7, null, null, 9, 3],
            Solution::invert_tree(binarytree![4, 7, 1, 3, 9])
        );

        assert_eq!(
            binarytree![4, null, 7],
            Solution::invert_tree(binarytree![4, 7])
        );

        assert_eq!(binarytree![4], Solution::invert_tree(binarytree![4]));
    }
}
