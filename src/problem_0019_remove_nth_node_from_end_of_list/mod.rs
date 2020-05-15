use super::data_structures::ListNode;

pub mod measure_length;
pub mod scout;

pub trait Solution {
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 3, 4, 5] as &[_], 2), &[1, 2, 3, 5] as &[_])];

        for ((head, n), expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::iter_list(&S::remove_nth_from_end(
                    test_utilities::make_list(head.iter().copied()),
                    n
                ))
                .copied()
                .collect::<Box<_>>()
                .as_ref(),
                expected
            );
        }
    }
}
