pub mod greedy;

pub trait Solution {
    fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 2, 5), [1, 2]),
            ((4, 3, 2), [0, 0]),
            ((3, 5, 1), [1, 2]),
            ((1, 7, 2), [1, 4]),
            ((7, 4, 1), [2, 4]),
        ];

        for ((a, b, c), expected) in test_cases {
            assert_eq!(S::num_moves_stones(a, b, c), expected);
        }
    }
}
