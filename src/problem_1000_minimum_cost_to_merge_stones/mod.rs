pub mod dynamic_programming;

pub trait Solution {
    fn merge_stones(stones: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 4, 1] as &[_], 2), 20),
            ((&[3, 2, 4, 1], 3), -1),
            ((&[3, 5, 1, 2, 6], 3), 25),
            ((&[6, 4, 4, 6], 2), 40),
        ];

        for ((stones, k), expected) in test_cases {
            assert_eq!(S::merge_stones(stones.to_vec(), k), expected);
        }
    }
}
