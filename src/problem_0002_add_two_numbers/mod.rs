use super::data_structures::ListNode;

pub mod simple;

pub trait Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 4, 3], &[5, 6, 4]), &[7, 0, 8])];

        for ((l1, l2), expected) in test_cases.iter().copied() {
            let l1 = test_utilities::make_list(l1.iter().copied());
            let l2 = test_utilities::make_list(l2.iter().copied());
            let expected = test_utilities::make_list(expected.iter().copied());

            assert_eq!(S::add_two_numbers(l1, l2), expected);
        }
    }
}
