pub mod counting_sort;

pub trait Solution {
    fn custom_sort_string(order: String, s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;
    use std::str;

    pub fn run<S: Solution>() {
        let test_cases = [(("cba", "abcd"), ("cba", "d")), (("cbafg", "abcd"), ("cba", "d"))];

        for ((order, s), (expected_ordered, expected_unordered)) in test_cases {
            let result = S::custom_sort_string(order.to_string(), s.to_string());
            let (ordered, unordered) = result.chars().partition::<String, _>(|&c| order.contains(c));

            assert_eq!(ordered, expected_ordered);

            assert_eq!(
                str::from_utf8(&test_utilities::unstable_sorted(unordered.bytes())).unwrap(),
                expected_unordered
            );
        }
    }
}
