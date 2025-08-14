pub mod greedy_binary_heap;
pub mod greedy_binary_heap_2;

pub trait Solution {
    fn max_spending(values: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[8, 5, 2], [6, 4, 1], [9, 7, 3]] as &dyn Matrix<_>, 285),
            (&[[10, 8, 6, 4, 2], [9, 7, 5, 3, 2]], 386),
        ];

        for (values, expected) in test_cases {
            assert_eq!(S::max_spending(values.to_vec()), expected);
        }
    }
}
