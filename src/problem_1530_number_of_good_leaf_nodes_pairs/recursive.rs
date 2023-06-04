use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn helper(node: &TreeNode, distance_minus_2: usize, result: &mut i32) -> [u16; 9] {
        assert!(distance_minus_2 < 9);

        let mut distances = match (
            node.left.as_deref().map(RefCell::borrow),
            node.right.as_deref().map(RefCell::borrow),
        ) {
            (None, None) => return [1, 0, 0, 0, 0, 0, 0, 0, 0],
            (None, Some(child)) | (Some(child), None) => {
                let mut distances = Self::helper(&child, distance_minus_2, result);

                for i in (0..distance_minus_2).rev() {
                    distances[i + 1] = distances[i];
                }

                distances
            }
            (Some(left), Some(right)) => {
                let mut left_distances = Self::helper(&left, distance_minus_2, result);
                let right_distances = Self::helper(&right, distance_minus_2, result);
                let mut total_left_count = 0;

                for i in 0..=distance_minus_2 {
                    total_left_count += left_distances[i];
                    *result += i32::from(total_left_count) * i32::from(right_distances[distance_minus_2 - i]);
                }

                for i in (0..distance_minus_2).rev() {
                    left_distances[i + 1] = left_distances[i] + right_distances[i];
                }

                left_distances
            }
        };

        distances[0] = 0;

        distances
    }

    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let distance = distance as u32;
        let mut result = 0;

        if distance >= 2 {
            Self::helper(&root.unwrap().borrow(), distance as usize - 2, &mut result);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        Self::count_pairs(root, distance)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
