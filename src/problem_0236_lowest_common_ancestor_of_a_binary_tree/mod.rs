#![allow(clippy::unnecessary_wraps)]

use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::data_structures::TreeNode;
    use crate::test_utilities;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn find_node(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref().and_then(|root| {
            let root_ref = root.borrow();

            if root_ref.val == val {
                Some(Rc::clone(root))
            } else {
                find_node(&root_ref.left, val).or_else(|| find_node(&root_ref.right, val))
            }
        })
    }

    fn inverted(root: Option<&RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|root| {
            let root = root.borrow();

            Rc::new(RefCell::new(TreeNode {
                val: root.val,
                left: inverted(root.left.as_deref()),
                right: inverted(root.right.as_deref()),
            }))
        })
    }

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[Some(3), Some(5), Some(1)] as &[_], 5, 1), 3),
            (
                (
                    &[
                        Some(3),
                        Some(5),
                        Some(1),
                        Some(6),
                        Some(2),
                        Some(0),
                        Some(8),
                        None,
                        None,
                        Some(7),
                        Some(4),
                    ],
                    5,
                    4,
                ),
                5,
            ),
            ((&[Some(1), Some(2)], 1, 2), 1),
            ((&[Some(1), Some(2), Some(3)], 3, 2), 1),
            ((&[Some(1), Some(2), Some(3), None, Some(4)], 4, 3), 1),
            (
                (&[Some(-1), Some(0), None, Some(1), None, Some(2), None, Some(3)], 3, 2),
                2,
            ),
            (
                (
                    &[Some(-1), Some(0), Some(3), Some(-2), Some(4), None, None, Some(8)],
                    8,
                    4,
                ),
                0,
            ),
            (
                (
                    &[
                        Some(9),
                        Some(-1),
                        Some(-4),
                        Some(10),
                        Some(3),
                        None,
                        None,
                        None,
                        Some(5),
                    ],
                    3,
                    5,
                ),
                -1,
            ),
            (
                (&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8)], 0, 8),
                1,
            ),
        ];

        for ((root, p, q), expected) in test_cases {
            let root = test_utilities::make_tree(root.iter().copied());
            let inverted_root = inverted(root.as_deref());

            for (root, p, q) in [(root, p, q), (inverted_root, q, p)] {
                let p = find_node(&root, p);
                let q = find_node(&root, q);
                let expected = find_node(&root, expected);

                assert_eq!(S::lowest_common_ancestor(root, p, q), expected);
            }
        }
    }
}
