use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    fn helper(
        root: i32,
        preorder: &[i32],
        postorder: &[i32],
        offset: usize,
        indices: &HashMap<i32, usize>,
    ) -> Rc<RefCell<TreeNode>> {
        let (left, right) = preorder.split_first().map_or((None, None), |(&left_root, rest)| {
            let i = indices[&left_root] - offset;
            let left = Self::helper(left_root, &rest[..i], postorder, offset, indices);

            let right = rest[i..].split_first().map(|(&right_root, rest)| {
                Self::helper(right_root, rest, &postorder[i + 1..], offset + i + 1, indices)
            });

            (Some(left), right)
        });

        Rc::new(RefCell::new(TreeNode { val: root, left, right }))
    }

    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        preorder.split_first().map(|(&root, rest)| {
            let indices = postorder.iter().copied().zip(0..).collect::<HashMap<_, _>>();

            Self::helper(root, rest, &postorder, 0, &indices)
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_from_pre_post(preorder, postorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
