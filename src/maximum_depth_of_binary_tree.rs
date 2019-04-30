use super::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(v) => {
                return 1 + std::cmp::max(
                    Self::max_depth(v.borrow().left.clone()),
                    Self::max_depth(v.borrow().right.clone()),
                );
            }
            None => 0,
        }
    }

    pub fn max_depth_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(v) => Self::depth_of_tree(Some(v), 1),
            None => 0,
        }
    }
    fn depth_of_tree(root: Option<Rc<RefCell<TreeNode>>>, level: i32) -> i32 {
        match root {
            Some(v) => match (v.borrow().left.clone(), v.borrow().right.clone()) {
                (Some(l), Some(r)) => {
                    let ld = Self::depth_of_tree(Some(l), level + 1);
                    let rd = Self::depth_of_tree(Some(r), level + 1);
                    return ld.max(rd);
                }
                (Some(l), None) => {
                    let ld = Self::depth_of_tree(Some(l), level + 1);
                    return ld;
                }
                (None, Some(r)) => {
                    let rd = Self::depth_of_tree(Some(r), level + 1);
                    return rd;
                }
                (_, _) => level,
            },
            None => level,
        }
    }

    pub fn max_depth_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::depth_of_tree_2(root, 1, &mut max);
        max
    }
    fn depth_of_tree_2(root: Option<Rc<RefCell<TreeNode>>>, level: i32, max: &mut i32) {
        match root {
            Some(v) => {
                *max = i32::max(*max, level);
                Self::depth_of_tree_2(v.borrow().left.clone(), level + 1, max);
                Self::depth_of_tree_2(v.borrow().right.clone(), level + 1, max)
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn max_depth() {
        assert_eq!(0, Solution::max_depth(binarytree![]));
        assert_eq!(1, Solution::max_depth(binarytree![1]));
        assert_eq!(2, Solution::max_depth(binarytree![1, 2]));
        assert_eq!(3, Solution::max_depth(binarytree![1, 2, 3, 4]));
        assert_eq!(
            3,
            Solution::max_depth(binarytree![3, 9, 20, null, null, 15, 7])
        );
    }

    #[test]
    fn max_depth_1() {
        assert_eq!(0, Solution::max_depth_1(binarytree![]));
        assert_eq!(1, Solution::max_depth_1(binarytree![1]));
        assert_eq!(2, Solution::max_depth_1(binarytree![1, 2]));
        assert_eq!(3, Solution::max_depth_1(binarytree![1, 2, 3, 4]));
        assert_eq!(
            3,
            Solution::max_depth_1(binarytree![3, 9, 20, null, null, 15, 7])
        );
    }

    #[test]
    fn max_depth_2() {
        assert_eq!(0, Solution::max_depth_2(binarytree![]));
        assert_eq!(1, Solution::max_depth_2(binarytree![1]));
        assert_eq!(2, Solution::max_depth_2(binarytree![1, 2]));
        assert_eq!(3, Solution::max_depth_2(binarytree![1, 2, 3, 4]));
        assert_eq!(
            3,
            Solution::max_depth_2(binarytree![3, 9, 20, null, null, 15, 7])
        );
    }

}
