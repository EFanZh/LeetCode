pub mod combinations;
pub mod dynamic_programming;

pub trait Solution {
    fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 2, 2, 3, 3, 4, 4, 5, 5] as &[_], 8), 20),
            ((&[1, 1, 2, 2, 2, 2], 5), 12),
            ((&[1, 0, 1, 0, 2, 1, 2], 1), 3),
            ((&[0, 0, 0], 0), 1),
            ((&[3, 3, 2, 0, 2], 7), 2),
        ];

        for ((arr, target), expected) in test_cases {
            assert_eq!(S::three_sum_multi(arr.to_vec(), target), expected);
        }
    }
}
