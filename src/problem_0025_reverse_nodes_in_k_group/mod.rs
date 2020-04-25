use super::data_structures::ListNode;

pub mod reverse_group;
pub mod reverse_group_2;

pub trait Solution {
    fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 2), &[2, 1, 4, 3, 5] as &[_]),
            ((&[1, 2, 3, 4, 5] as &[_], 3), &[3, 2, 1, 4, 5] as &[_]),
        ];

        for ((head, k), expected) in test_cases.iter().copied() {
            let head = test_utilities::make_list(head.iter().copied());

            let result = test_utilities::iter_list(&S::reverse_k_group(head, k))
                .copied()
                .collect::<Box<_>>();

            assert_eq!(result.as_ref(), expected);
        }
    }
}
