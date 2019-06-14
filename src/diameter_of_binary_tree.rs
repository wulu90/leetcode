/**
 * 543. Diameter of Binary Tree
 *
 * Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
 *
 * Example:
 * Given a binary tree
 *           1
 *          / \
 *         2   3
 *        / \     
 *       4   5    
 * Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
 *
 * Note: The length of path between two nodes is represented by the number of edges between them.
 */

use crate::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 1;
        Self::depth(root, &mut res);
        res - 1
    }

    fn depth(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let l = Self::depth(node.borrow().left.clone(), res);
                let r = Self::depth(node.borrow().right.clone(), res);
                *res = std::cmp::max(*res, l + r + 1);
                std::cmp::max(l, r) + 1
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn diameter_of_binary_tree() {
        assert_eq!(
            3,
            Solution::diameter_of_binary_tree(binarytree![1, 2, 3, 4, 5])
        );
    }
}