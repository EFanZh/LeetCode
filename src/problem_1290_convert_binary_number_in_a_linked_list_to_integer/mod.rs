use crate::data_structures::ListNode;

pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn get_decimal_value(head: Option<Box<ListNode>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 0, 1] as &[_], 5), (&[0], 0)];

        for (head, expected) in test_cases {
            assert_eq!(
                S::get_decimal_value(test_utilities::make_list(head.iter().copied())),
                expected
            );
        }
    }
}
