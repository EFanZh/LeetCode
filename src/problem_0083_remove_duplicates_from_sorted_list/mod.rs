use crate::data_structures::ListNode;

pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 2] as &[_], &[1, 2] as &[_]), (&[1, 1, 2, 3, 3], &[1, 2, 3])];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(S::delete_duplicates(test_utilities::make_list(head.iter().copied())))
                    .collect::<Box<_>>()
                    .as_ref(),
                expected,
            );
        }
    }
}
