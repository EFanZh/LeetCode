use crate::data_structures::ListNode;

pub mod heap;

pub trait Solution {
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[&[1, 4, 5] as &[_], &[1, 3, 4], &[2, 6]] as &[&[_]],
            &[1, 1, 2, 3, 4, 4, 5, 6] as &[_],
        )];

        for (lists, expected) in test_cases {
            let lists = lists
                .iter()
                .map(|list| test_utilities::make_list(list.iter().copied()))
                .collect();

            let result = test_utilities::iter_list(S::merge_k_lists(lists)).collect::<Box<_>>();

            assert_eq!(result.as_ref(), expected);
        }
    }
}
