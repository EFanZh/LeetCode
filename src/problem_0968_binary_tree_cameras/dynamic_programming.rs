use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

struct State {
    directly: u32,
    indirectly: u32,
    children_only: u32,
}

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>) -> State {
        node.map_or(
            State {
                directly: u32::MAX / 2,
                indirectly: 0,
                children_only: 0,
            },
            |node| {
                let node = node.borrow();
                let left_state = Self::helper(node.left.as_deref());
                let left_monitored = left_state.directly.min(left_state.indirectly);
                let right_state = Self::helper(node.right.as_deref());
                let right_monitored = right_state.directly.min(right_state.indirectly);

                State {
                    directly: left_monitored.min(left_state.children_only)
                        + right_monitored.min(right_state.children_only)
                        + 1,
                    indirectly: (left_state.directly + right_monitored).min(right_state.directly + left_monitored),
                    children_only: (left_state.indirectly + right_state.indirectly),
                }
            },
        )
    }

    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let state = Self::helper(root.as_deref());

        state.directly.min(state.indirectly) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_camera_cover(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
