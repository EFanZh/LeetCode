use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(mut prev) = root.clone() {
            let (mut current, right) = {
                let mut perv = prev.borrow_mut();

                (perv.left.take(), perv.right.take())
            };

            let mut stack = vec![right];

            loop {
                if let Some(node) = current {
                    let new_prev = Rc::clone(prev.borrow_mut().right.get_or_insert(node));

                    prev = new_prev;

                    let (left, right) = {
                        let mut prev = prev.borrow_mut();

                        (prev.left.take(), prev.right.take())
                    };

                    stack.push(right);

                    current = left;
                } else if let Some(right) = stack.pop() {
                    current = right;
                } else {
                    break;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::flatten(root);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
