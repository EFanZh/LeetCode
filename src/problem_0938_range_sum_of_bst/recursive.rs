use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    fn sum(node: Option<&RefCell<TreeNode>>, result: &mut i32) {
        if let Some(node) = node.map(RefCell::borrow) {
            *result += node.val;

            Self::sum(node.left.as_deref(), result);
            Self::sum(node.right.as_deref(), result);
        }
    }

    fn sum_low(node: Option<&RefCell<TreeNode>>, low: i32, result: &mut i32) {
        if let Some(node) = node.map(RefCell::borrow) {
            match low.cmp(&node.val) {
                Ordering::Less => {
                    *result += node.val;

                    Self::sum(node.right.as_deref(), result);
                    Self::sum_low(node.left.as_deref(), low, result);
                }
                Ordering::Equal => {
                    *result += node.val;

                    Self::sum(node.right.as_deref(), result);
                }
                Ordering::Greater => Self::sum_low(node.right.as_deref(), low, result),
            }
        }
    }

    fn sum_high(node: Option<&RefCell<TreeNode>>, high: i32, result: &mut i32) {
        if let Some(node) = node.map(RefCell::borrow) {
            match high.cmp(&node.val) {
                Ordering::Less => Self::sum_high(node.left.as_deref(), high, result),
                Ordering::Equal => {
                    *result += node.val;

                    Self::sum(node.left.as_deref(), result);
                }
                Ordering::Greater => {
                    *result += node.val;

                    Self::sum(node.left.as_deref(), result);
                    Self::sum_high(node.right.as_deref(), high, result);
                }
            }
        }
    }

    fn sum_range(node: Option<&RefCell<TreeNode>>, low: i32, high: i32, result: &mut i32) {
        if let Some(node) = node.map(RefCell::borrow) {
            match low.cmp(&node.val) {
                Ordering::Less => match high.cmp(&node.val) {
                    Ordering::Less => Self::sum_range(node.left.as_deref(), low, high, result),
                    Ordering::Equal => {
                        *result += node.val;

                        Self::sum_low(node.left.as_deref(), low, result);
                    }
                    Ordering::Greater => {
                        *result += node.val;

                        Self::sum_low(node.left.as_deref(), low, result);
                        Self::sum_high(node.right.as_deref(), high, result);
                    }
                },
                Ordering::Equal => {
                    *result += node.val;

                    if high > node.val {
                        Self::sum_high(node.right.as_deref(), high, result);
                    }
                }
                Ordering::Greater => Self::sum_range(node.right.as_deref(), low, high, result),
            }
        }
    }

    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut result = 0;

        Self::sum_range(root.as_deref(), low, high, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Self::range_sum_bst(root, low, high)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
