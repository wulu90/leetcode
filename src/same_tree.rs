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
    }
}
