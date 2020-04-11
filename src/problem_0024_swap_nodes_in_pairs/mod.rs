use super::data_structures::ListNode;

pub mod obvious;

pub trait Solution {
    fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4] as &[_], &[2, 1, 4, 3] as &[_]), (&[1], &[1])];

        for (head, expected) in test_cases.iter().copied() {
            let head = test_utilities::make_list(head.iter().copied());

            let result = test_utilities::iter_list(&S::swap_pairs(head))
                .copied()
                .collect::<Box<_>>();

            assert_eq!(result.as_ref(), expected);
        }
    }
}
