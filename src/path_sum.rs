use super::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let mut has = false;
        Self::path_sum_dfs(root.clone(), 0, sum, &mut has);
        has
    }

    fn path_sum_dfs(root: Option<Rc<RefCell<TreeNode>>>, mut temp: i32, sum: i32, has: &mut bool) {
        if let Some(v) = root {
            let val = v.borrow().val;
            temp += val;
            if v.borrow().left.is_none() && v.borrow().right.is_none() && sum == temp {
                *has = true;
            }
            if v.borrow().left.is_some() {
                Self::path_sum_dfs(v.borrow().left.clone(), temp, sum, has);
            }
            if v.borrow().right.is_some() {
                Self::path_sum_dfs(v.borrow().right.clone(), temp, sum, has);
            }
        }
    }

    pub fn has_path_sum_bfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut queue = VecDeque::new();
        queue.push_back((root.clone(), 0));

        while !queue.is_empty() {
            let cur = queue.pop_front();

            if let Some((Some(node), v)) = cur {
                let nodeval = node.borrow().val;
                if node.borrow().left.is_none()
                    && node.borrow().right.is_none()
                    && v + nodeval == sum
                {
                    return true;
                }
                if node.borrow().left.is_some() {
                    queue.push_back((node.borrow().left.clone(), v + nodeval))
                }
                if node.borrow().right.is_some() {
                    queue.push_back((node.borrow().right.clone(), v + nodeval));
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn has_path_sum() {
        assert_eq!(
            true,
            Solution::has_path_sum(
                binarytree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                22
            )
        );
    }

    #[test]
    fn has_path_sum_bfs() {
        assert_eq!(
            true,
            Solution::has_path_sum_bfs(
                binarytree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                22
            )
        );
    }

}
