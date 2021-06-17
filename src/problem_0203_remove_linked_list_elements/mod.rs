use super::data_structures::ListNode;

// pub mod iterative; // TODO: One day this one will work.
pub mod tail_recursive;

pub trait Solution {
    fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 6, 3, 4, 5, 6] as &[_], 6), &[1, 2, 3, 4, 5] as &[_])];

        for ((head, val), expected) in test_cases {
            assert_eq!(
                test_utilities::iter_list(&S::remove_elements(
                    test_utilities::make_list(head.iter().copied()),
                    val
                ))
                .copied()
                .collect::<Box<_>>()
                .as_ref(),
                expected
            );
        }
    }
}
