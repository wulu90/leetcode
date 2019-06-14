/**
 * 538. Convert BST to Greater Tree
 *
 * Given a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original
 * BST is changed to the original key plus sum of all keys greater than the original key in BST.
 *
 * Example:
 *
 * Input: The root of a Binary Search Tree like this:
 *               5
 *             /   \
 *            2     13
 *
 * Output: The root of a Greater Tree like this:
 *              18
 *             /   \
 *           20     13
 */

use crate::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::helper(root.clone(), &mut sum);
        root
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_some() {
            Self::helper(root.as_ref().unwrap().borrow().right.clone(), sum);
            *sum += root.as_ref().unwrap().borrow().val;
            root.as_ref().unwrap().borrow_mut().val = *sum;
            Self::helper(root.as_ref().unwrap().borrow().left.clone(), sum);
        }
        root
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn convert_bst() {
        assert_eq!(
            binarytree![18, 20, 13],
            Solution::convert_bst(binarytree![5, 2, 13])
        );
    }
}