use super::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn is_palindrome(head: Option<Box<ListNode>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2] as &[_], false), (&[1, 2, 2, 1], true), (&[1], true)];

        for (head, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::is_palindrome(test_utilities::make_list(head.iter().copied())),
                expected
            );
        }
    }
}
