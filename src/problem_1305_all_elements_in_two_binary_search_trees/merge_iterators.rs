use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

impl Solution {
    fn iter_tree(mut node: Option<Rc<RefCell<TreeNode>>>) -> impl Iterator<Item = i32> {
        let mut stack = Vec::new();

        iter::from_fn(move || {
            while let Some(node_cell) = node.as_deref() {
                let node_ref = node_cell.borrow();
                let value = node_ref.val;
                let left = node_ref.left.clone();
                let right = node_ref.right.clone();

                drop(node_ref);

                stack.push((value, right));

                node = left;
            }

            stack.pop().map(|(value, right)| {
                node = right;

                value
            })
        })
    }

    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut iter_1 = Self::iter_tree(root1);
        let mut iter_2 = Self::iter_tree(root2);
        let mut result = Vec::new();

        let extra = if let Some(mut left) = iter_1.next() {
            'outer: while let Some(right) = iter_2.next() {
                loop {
                    if right < left {
                        result.push(right);

                        break;
                    }

                    result.push(left);

                    if let Some(new_left) = iter_1.next() {
                        left = new_left;
                    } else {
                        left = right;
                        iter_1 = iter_2;

                        break 'outer;
                    }
                }
            }

            result.push(left);

            iter_1
        } else {
            iter_2
        };

        result.extend(extra);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::get_all_elements(root1, root2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
