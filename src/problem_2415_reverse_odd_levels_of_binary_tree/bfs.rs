use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::from([Rc::clone(root.as_ref().unwrap())]);

        'outer: loop {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                if let (Some(left), Some(right)) = (&node.left, &node.right) {
                    queue.extend([Rc::clone(left), Rc::clone(right)]);
                } else {
                    break 'outer;
                }
            }

            let mut iter = queue.iter();

            while let (Some(left), Some(right)) = (iter.next(), iter.next_back()) {
                mem::swap(&mut left.borrow_mut().val, &mut right.borrow_mut().val);
            }

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                if let (Some(left), Some(right)) = (&node.left, &node.right) {
                    queue.extend([Rc::clone(left), Rc::clone(right)]);
                } else {
                    break 'outer;
                }
            }
        }

        root
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::reverse_odd_levels(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
