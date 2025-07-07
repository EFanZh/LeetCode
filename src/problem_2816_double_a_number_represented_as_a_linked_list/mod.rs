use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 8, 9] as &[_], &[3, 7, 8] as &[_]), (&[9, 9, 9], &[1, 9, 9, 8])];

        for (head, expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(S::double_it(test_utilities::make_list(head.iter().copied())))
                    .collect::<Vec<_>>(),
                expected,
            );
        }
    }
}
