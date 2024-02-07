use crate::data_structures::ListNode;

pub mod in_place;
pub mod reverse_half;

pub trait Solution {
    fn pair_sum(head: Option<Box<ListNode>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 4, 2, 1] as &[_], 6), (&[4, 2, 2, 3], 7), (&[1, 100_000], 100_001)];

        for (head, expected) in test_cases {
            assert_eq!(S::pair_sum(test_utilities::make_list(head.iter().copied())), expected);
        }
    }
}
