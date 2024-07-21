use crate::data_structures::ListNode;

pub mod monotonic_stack;

pub trait Solution {
    fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1, 5] as &[_], &[5, 5, 0] as &[_]),
            (&[2, 7, 4, 3, 5], &[7, 0, 5, 5, 0]),
        ];

        for (head, expected) in test_cases {
            assert_eq!(
                S::next_larger_nodes(test_utilities::make_list(head.iter().copied())),
                expected,
            );
        }
    }
}
