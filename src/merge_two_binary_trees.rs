/*
 * 617. Merge Two Binary Trees
 *
 * Given two binary trees and imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not.
 *
 * You need to merge them into a new binary tree. The merge rule is that if two nodes overlap,
 * then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of new tree.
 *
 * Example 1:
 * Input:
 * 	Tree 1                     Tree 2
 *           1                         2
 *          / \                       / \
 *         3   2                     1   3
 *        /                           \   \
 *       5                             4   7
 * Output:
 * Merged tree:
 * 	     3
 * 	    / \
 * 	   4   5
 * 	  / \   \
 * 	 5   4   7
 *
 * Note: The merging process must start from the root nodes of both trees.
 *
 */

use crate::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if t1.is_none() {
            return t2;
        }
        if t2.is_none() {
            return t1;
        }
        let mut res;
        res = TreeNode::new(t1.as_ref().unwrap().borrow().val + t2.as_ref().unwrap().borrow().val);

        res.left = Self::merge_trees(
            t1.as_ref().unwrap().borrow().left.clone(),
            t2.as_ref().unwrap().borrow().left.clone(),
        );
        res.right = Self::merge_trees(
            t1.as_ref().unwrap().borrow().right.clone(),
            t2.as_ref().unwrap().borrow().right.clone(),
        );

        Some(Rc::new(RefCell::new(res)))
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn merge_trees() {
        assert_eq!(
            binarytree![3, 4, 5, 5, 4, null, 7],
            Solution::merge_trees(
                binarytree![1, 3, 2, 5],
                binarytree![2, 1, 3, null, 4, null, 7]
            )
        );
    }
}