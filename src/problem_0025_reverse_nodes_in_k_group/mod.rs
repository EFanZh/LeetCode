use crate::data_structures::ListNode;

pub mod reverse_group;
pub mod reverse_group_2;

pub trait Solution {
    fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 2), &[2, 1, 4, 3, 5] as &[_]),
            ((&[1, 2, 3, 4, 5], 3), &[3, 2, 1, 4, 5]),
        ];

        for ((head, k), expected) in test_cases {
            let head = test_utilities::make_list(head.iter().copied());

            let result = test_utilities::iter_list(&S::reverse_k_group(head, k))
                .copied()
                .collect::<Box<_>>();

            assert_eq!(result.as_ref(), expected);
        }
    }
}
