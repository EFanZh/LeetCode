use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

enum State {
    Zero,
    One(i32),
    Two(i32, i32),
}

impl Solution {
    fn helper(node: Option<&RefCell<TreeNode>>, state: &mut State) {
        if let Some(node) = node {
            let node = node.borrow();

            match *state {
                State::Zero => *state = State::One(node.val),
                State::One(x) => match node.val.cmp(&x) {
                    Ordering::Less => *state = State::Two(node.val, x),
                    Ordering::Equal => {}
                    Ordering::Greater => *state = State::Two(x, node.val),
                },
                State::Two(x, y) => match node.val.cmp(&x) {
                    Ordering::Less => *state = State::Two(node.val, x),
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        if node.val < y {
                            *state = State::Two(x, node.val);
                        }
                    }
                },
            }

            Self::helper(node.left.as_deref(), state);
            Self::helper(node.right.as_deref(), state);
        }
    }

    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut state = State::Zero;

        Self::helper(root.as_deref(), &mut state);

        if let State::Two(_, y) = state {
            y
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find_second_minimum_value(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
