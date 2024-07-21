use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1] as &[_], [-1, -1]),
            (&[5, 3, 1, 2, 5, 1, 2], [1, 3]),
            (&[1, 3, 2, 2, 3, 2, 2, 2, 7], [3, 3]),
            (&[2, 2, 1, 3], [-1, -1]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                S::nodes_between_critical_points(test_utilities::make_list(head.iter().copied())),
                expected,
            );
        }
    }
}
