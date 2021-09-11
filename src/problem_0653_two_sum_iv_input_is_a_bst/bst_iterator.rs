use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    fn iter_tree(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        order: impl Fn(&TreeNode) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) + Copy,
    ) -> impl FnMut() -> i32 {
        fn next(
            stack: &mut Vec<(i32, Option<Rc<RefCell<TreeNode>>>)>,
            node: &mut Option<Rc<RefCell<TreeNode>>>,
            order: impl Fn(&TreeNode) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>),
        ) -> i32 {
            loop {
                if let Some(node_rc) = node.as_deref() {
                    let (value, (first, last)) = {
                        let node_ref = node_rc.borrow();

                        (node_ref.val, order(&node_ref))
                    };

                    stack.push((value, last));
                    *node = first;
                } else {
                    let (value, last) = stack.pop().unwrap();

                    *node = last;

                    return value;
                }
            }
        }

        let mut stack = Vec::new();

        move || next(&mut stack, &mut root, order)
    }

    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut iter_forward = Self::iter_tree(root.clone(), |node| (node.left.clone(), node.right.clone()));
        let mut iter_backward = Self::iter_tree(root, |node| (node.right.clone(), node.left.clone()));
        let mut left = iter_forward();
        let mut right = iter_backward();

        while left < right {
            match (left + right).cmp(&k) {
                Ordering::Less => left = iter_forward(),
                Ordering::Equal => return true,
                Ordering::Greater => right = iter_backward(),
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        Self::find_target(root, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
