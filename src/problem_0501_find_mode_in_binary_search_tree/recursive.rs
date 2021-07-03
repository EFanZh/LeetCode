use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    fn iterate_tree(root: Option<&RefCell<TreeNode>>, f: &mut impl FnMut(i32)) {
        if let Some(node) = root {
            let node_ref = node.borrow();

            Self::iterate_tree(node_ref.left.as_deref(), f);
            f(node_ref.val);
            Self::iterate_tree(node_ref.right.as_deref(), f);
        }
    }

    fn iterate_chunks(root: Option<&RefCell<TreeNode>>, f: &mut impl FnMut(i32, usize)) {
        let mut prev = None;
        let mut length = 0;

        Self::iterate_tree(root, &mut |val| {
            if let Some(prev_val) = prev {
                if val == prev_val {
                    length += 1;
                } else {
                    f(prev_val, length);

                    prev = Some(val);
                    length = 1;
                }
            } else {
                prev = Some(val);
                length = 1;
            }
        });

        if let Some(prev_val) = prev {
            f(prev_val, length);
        }
    }

    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut max_length = 0;
        let mut max_length_count = 0;

        Self::iterate_chunks(root.as_deref(), &mut |_, length| match length.cmp(&max_length) {
            Ordering::Less => {}
            Ordering::Equal => max_length_count += 1,
            Ordering::Greater => {
                max_length = length;
                max_length_count = 1;
            }
        });

        let mut result = Vec::with_capacity(max_length_count);

        Self::iterate_chunks(root.as_deref(), &mut |val, length| {
            if length == max_length {
                result.push(val);
            }
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::find_mode(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
