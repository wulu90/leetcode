use crate::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }

    pub fn is_same_tree1(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(a), Some(b)) => {
                a.borrow().val == b.borrow().val
                    && Self::is_same_tree1(a.borrow().left.clone(), b.borrow().left.clone())
                    && Self::is_same_tree1(a.borrow().right.clone(), b.borrow().right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn is_same_tree() {
        assert_eq!(
            true,
            Solution::is_same_tree(binarytree![1, 2], binarytree![1, 2])
        );

        assert_eq!(
            false,
            Solution::is_same_tree(binarytree![1, 2], binarytree![1, null, 2])
        );

        assert_eq!(
            false,
            Solution::is_same_tree(binarytree![1, 2, 1], binarytree![1, 1, 2])
        );
        assert_eq!(true, Solution::is_same_tree(binarytree![], binarytree![]));
    }

    #[test]
    fn is_same_tree1() {
        assert_eq!(
            true,
            Solution::is_same_tree1(binarytree![1, 2], binarytree![1, 2])
        );

        assert_eq!(
            false,
            Solution::is_same_tree1(binarytree![1, 2], binarytree![1, null, 2])
        );

        assert_eq!(
            false,
            Solution::is_same_tree1(binarytree![1, 2, 1], binarytree![1, 1, 2])
        );
        assert_eq!(true, Solution::is_same_tree1(binarytree![], binarytree![]));
    }
}
