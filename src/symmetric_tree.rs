use super::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(v) => {
                Self::left_symmetric_right(v.borrow().left.clone(), v.borrow().right.clone())
            }
            None => true,
        }
    }
    fn left_symmetric_right(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (Some(a), Some(b)) => {
                a.borrow().val == b.borrow().val
                    && Self::left_symmetric_right(a.borrow().left.clone(), b.borrow().right.clone())
                    && Self::left_symmetric_right(a.borrow().right.clone(), b.borrow().left.clone())
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
    fn is_symmetric() {
        assert_eq!(
            true,
            Solution::is_symmetric(binarytree![1, 2, 2, 3, 4, 4, 3])
        );
        assert_eq!(
            false,
            Solution::is_symmetric(binarytree![1, 2, 2, null, 3, null, 3])
        );

        assert_eq!(true, Solution::is_symmetric(binarytree![1]));

        assert_eq!(true, Solution::is_symmetric(binarytree![]));

        assert_eq!(false, Solution::is_symmetric(binarytree![1, 2]));

        assert_eq!(false, Solution::is_symmetric(binarytree![1, 2, 3]));
    }

}
