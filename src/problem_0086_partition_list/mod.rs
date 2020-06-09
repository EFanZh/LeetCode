use super::data_structures::ListNode;

pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 4, 3, 2, 5, 2] as &[_], 3), &[1, 2, 2, 4, 3, 5] as &[_])];

        for ((head, x), expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::iter_list(&S::partition(test_utilities::make_list(head.iter().copied()), x))
                    .copied()
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
