use super::data_structures::TreeNode;
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
    use super::super::data_structures::TreeNode;
    use super::super::test_utilities;
    use super::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn find_node(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref().and_then(|root| {
            let root_ref = root.borrow();

            if root_ref.val == val {
                Some(root.clone())
            } else {
                find_node(&root_ref.left, val).or_else(|| find_node(&root_ref.right, val))
            }
        })
    }

    pub fn run<S: Solution>() {
        let test_cases = [
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
                    ] as &[_],
                    5,
                    1,
                ),
                3,
            ),
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
        ];

        for ((root, p, q), expected) in test_cases.iter().copied() {
            let root = test_utilities::make_tree(root.iter().copied());
            let p = find_node(&root, p);
            let q = find_node(&root, q);
            let expected = find_node(&root, expected);

            assert_eq!(S::lowest_common_ancestor(root, p, q), expected);
        }
    }
}
