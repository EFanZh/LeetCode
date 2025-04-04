use crate::data_structures::ListNode;

pub mod zip;
pub mod zip_2;

pub trait Solution {
    fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 4] as &[_], &[1, 3, 4] as &[_]), &[1, 1, 2, 3, 4, 4] as &[_]),
            ((&[2], &[1]), &[1, 2]),
            ((&[], &[]), &[]),
        ];

        for ((l1, l2), expected) in test_cases {
            let l1 = test_utilities::make_list(l1.iter().copied());
            let l2 = test_utilities::make_list(l2.iter().copied());
            let result = S::merge_two_lists(l1, l2);

            assert_eq!(test_utilities::iter_list(result).collect::<Box<_>>().as_ref(), expected);
        }
    }
}
