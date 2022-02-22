use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::ops::Deref;
use std::rc::Rc;

trait RefTreeNodeEx: Deref<Target = TreeNode> + Sized {
    fn into_left(self) -> Option<Rc<RefCell<TreeNode>>> {
        self.left.clone()
    }

    fn into_right(self) -> Option<Rc<RefCell<TreeNode>>> {
        self.right.clone()
    }
}

impl RefTreeNodeEx for Ref<'_, TreeNode> {}

impl Solution {
    fn sum(mut node: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) {
        while let Some(node_ref) = node {
            let node_ref = node_ref.borrow();

            *result += node_ref.val;

            Self::sum(node_ref.left.clone(), result);

            node = node_ref.into_right();
        }
    }

    fn sum_low(mut node: Option<Rc<RefCell<TreeNode>>>, low: i32, result: &mut i32) {
        while let Some(node_ref) = node {
            let node_ref = node_ref.borrow();

            match low.cmp(&node_ref.val) {
                Ordering::Less => {
                    *result += node_ref.val;

                    Self::sum(node_ref.right.clone(), result);

                    node = node_ref.into_left();
                }
                Ordering::Equal => {
                    *result += node_ref.val;

                    Self::sum(node_ref.into_right(), result);

                    break;
                }
                Ordering::Greater => node = node_ref.into_right(),
            }
        }
    }

    fn sum_high(mut node: Option<Rc<RefCell<TreeNode>>>, high: i32, result: &mut i32) {
        while let Some(node_ref) = node {
            let node_ref = node_ref.borrow();

            match high.cmp(&node_ref.val) {
                Ordering::Less => node = node_ref.into_left(),
                Ordering::Equal => {
                    *result += node_ref.val;

                    Self::sum(node_ref.into_left(), result);

                    break;
                }
                Ordering::Greater => {
                    *result += node_ref.val;

                    Self::sum(node_ref.left.clone(), result);
                    node = node_ref.into_right();
                }
            }
        }
    }

    fn sum_range(mut node: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, result: &mut i32) {
        while let Some(node_ref) = node {
            let node_ref = node_ref.borrow();

            match low.cmp(&node_ref.val) {
                Ordering::Less => match high.cmp(&node_ref.val) {
                    Ordering::Less => node = node_ref.into_left(),
                    Ordering::Equal => {
                        *result += node_ref.val;

                        Self::sum_low(node_ref.into_left(), low, result);

                        break;
                    }
                    Ordering::Greater => {
                        *result += node_ref.val;

                        Self::sum_low(node_ref.left.clone(), low, result);
                        Self::sum_high(node_ref.into_right(), high, result);

                        break;
                    }
                },
                Ordering::Equal => {
                    *result += node_ref.val;

                    if high > node_ref.val {
                        Self::sum_high(node_ref.into_right(), high, result);
                    }

                    break;
                }
                Ordering::Greater => node = node_ref.into_right(),
            }
        }
    }

    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut result = 0;

        Self::sum_range(root, low, high, &mut result);

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
