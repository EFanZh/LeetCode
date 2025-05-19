use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::ops::ControlFlow;
use std::rc::Rc;
use std::{iter, mem};

impl Solution {
    fn find_1(
        node: Option<&RefCell<TreeNode>>,
        path: &mut Vec<u8>,
        value: i32,
        target: &mut Vec<u8>,
    ) -> ControlFlow<()> {
        if let Some(node) = node {
            let node = node.borrow();
            let node = &*node;

            if node.val == value {
                mem::swap(path, target);

                return ControlFlow::Break(());
            }

            Self::find_1_children(node, path, value, target)?;
        }

        ControlFlow::Continue(())
    }

    fn find_1_children(node: &TreeNode, path: &mut Vec<u8>, value: i32, target: &mut Vec<u8>) -> ControlFlow<()> {
        path.push(b'L');
        Self::find_1(node.left.as_deref(), path, value, target)?;
        path.pop();

        path.push(b'R');
        Self::find_1(node.right.as_deref(), path, value, target)?;
        path.pop();

        ControlFlow::Continue(())
    }

    fn find_2(
        node: Option<&RefCell<TreeNode>>,
        path: &mut Vec<u8>,
        value_1: i32,
        target_1: &mut Vec<u8>,
        value_2: i32,
        target_2: &mut Vec<u8>,
    ) -> ControlFlow<()> {
        if let Some(node) = node {
            let node = node.borrow();
            let node = &*node;

            let (found, value, target) = if node.val == value_1 {
                (target_1, value_2, target_2)
            } else if node.val == value_2 {
                (target_2, value_1, target_1)
            } else {
                path.push(b'L');
                Self::find_2(node.left.as_deref(), path, value_1, target_1, value_2, target_2)?;
                path.pop();

                path.push(b'R');
                Self::find_2(node.right.as_deref(), path, value_1, target_1, value_2, target_2)?;
                path.pop();

                return ControlFlow::Continue(());
            };

            drop(mem::replace(found, path.clone()));

            Self::find_1_children(node, path, value, target)?;
        }

        ControlFlow::Continue(())
    }

    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        let mut start_path = Vec::new();
        let mut dest_path = Vec::new();

        _ = Self::find_2(
            root.as_deref(),
            &mut Vec::new(),
            start_value,
            &mut start_path,
            dest_value,
            &mut dest_path,
        );

        let mut common_length = 0_usize;

        for (lhs, rhs) in start_path.iter().zip(&dest_path) {
            if lhs == rhs {
                common_length += 1;
            } else {
                break;
            }
        }

        dest_path.splice(..common_length, iter::repeat_n(b'U', start_path.len() - common_length));

        String::from_utf8(dest_path).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        Self::get_directions(root, start_value, dest_value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
