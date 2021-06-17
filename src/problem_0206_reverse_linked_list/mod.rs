use super::data_structures::ListNode;

pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4, 5] as &[_], &[5, 4, 3, 2, 1] as &[_])];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(&S::reverse_list(test_utilities::make_list(head.iter().copied())))
                    .copied()
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
