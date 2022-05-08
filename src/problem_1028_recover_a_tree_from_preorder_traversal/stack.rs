use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::<Rc<RefCell<TreeNode>>>::new();
        let mut traversal = traversal.as_bytes();

        while let Some(depth) = traversal.iter().position(|&c| c != b'-') {
            traversal = &traversal[depth..];

            let value_str_length = traversal.iter().position(|&c| c == b'-').unwrap_or(traversal.len());
            let (left, right) = traversal.split_at(value_str_length);
            let mut value = 0;

            for &c in left {
                value = value * 10 + i32::from(c - b'0');
            }

            let node = Rc::new(RefCell::new(TreeNode {
                val: value,
                left: None,
                right: None,
            }));

            if depth != 0 {
                stack.truncate(depth);

                let mut parent = stack.last().unwrap().borrow_mut();

                if parent.left.is_none() {
                    parent.left = Some(Rc::clone(&node));
                } else {
                    parent.right = Some(Rc::clone(&node));
                }
            }

            stack.push(node);

            traversal = right;
        }

        stack.into_iter().next()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recover_from_preorder(traversal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
