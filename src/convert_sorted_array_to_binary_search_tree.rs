use super::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        if nums.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
        }

        let mid = (nums.len()) / 2;
        let root = Some(Rc::new(RefCell::new(TreeNode::new(nums[mid]))));
        root.as_ref().unwrap().borrow_mut().left = Self::sorted_array_to_bst(nums[0..mid].to_vec());
        root.as_ref().unwrap().borrow_mut().right =
            Self::sorted_array_to_bst(nums[mid + 1..].to_vec());

        root
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;
    #[test]
    fn sorted_array_to_bst() {
        assert_eq!(
            binarytree![0, -3, 9, -10, null, 5],
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
        );

        assert_eq!(binarytree![-10], Solution::sorted_array_to_bst(vec![-10]));

        assert_eq!(binarytree![], Solution::sorted_array_to_bst(vec![]));

        assert_eq!(
            binarytree![0, -10, 6],
            Solution::sorted_array_to_bst(vec![-10, 0, 6])
        );
    }
}
