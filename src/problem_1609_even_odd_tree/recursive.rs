use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::ControlFlow;
use std::rc::Rc;

impl Solution {
    fn check(
        queue: &mut VecDeque<Rc<RefCell<TreeNode>>>,
        mut prev: i32,
        mut check: impl FnMut(i32, i32) -> bool,
    ) -> ControlFlow<bool> {
        for _ in 0..queue.len() {
            let parent = queue.pop_front().unwrap();
            let parent = parent.borrow();

            for child in [&parent.left, &parent.right].iter().copied().flatten() {
                let val = child.borrow().val;

                if check(prev, val) {
                    queue.push_back(Rc::clone(child));

                    prev = val;
                } else {
                    return ControlFlow::Break(false);
                }
            }
        }

        if queue.is_empty() {
            ControlFlow::Break(true)
        } else {
            ControlFlow::Continue(())
        }
    }

    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();

        if root.borrow().val & 1 == 0 {
            false
        } else {
            let mut queue = VecDeque::from([root]);

            loop {
                match Self::check(&mut queue, i32::MAX, |lhs, rhs| rhs & 1 == 0 && lhs > rhs) {
                    ControlFlow::Continue(()) => (),
                    ControlFlow::Break(result) => return result,
                }

                match Self::check(&mut queue, i32::MIN, |lhs, rhs| rhs & 1 != 0 && lhs < rhs) {
                    ControlFlow::Continue(()) => (),
                    ControlFlow::Break(result) => return result,
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_even_odd_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
