use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

enum State {
    Initial,
    SeenX(i32),
    SeenY(i32),
}

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut queue = VecDeque::from(vec![(root.unwrap(), -1)]);

        loop {
            let mut state = State::Initial;

            for _ in 0..queue.len() {
                let (node, parent) = queue.pop_front().unwrap();
                let node = node.borrow();

                match state {
                    State::Initial => {
                        if node.val == x {
                            state = State::SeenX(parent);
                        } else if node.val == y {
                            state = State::SeenY(parent);
                        }
                    }
                    State::SeenX(x_parent) => {
                        if node.val == y {
                            return parent != x_parent;
                        }
                    }
                    State::SeenY(y_parent) => {
                        if node.val == x {
                            return parent != y_parent;
                        }
                    }
                }

                if let Some(left) = node.left.clone() {
                    queue.push_back((left, node.val));
                }

                if let Some(right) = node.right.clone() {
                    queue.push_back((right, node.val));
                }
            }

            if !matches!(state, State::Initial) {
                return false;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        Self::is_cousins(root, x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
