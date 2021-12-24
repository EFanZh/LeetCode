pub mod iterative;

pub trait Solution {
    fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1] as &[_], &[2, 2] as &[_]), [1, 2]),
            ((&[1, 2], &[2, 3]), [1, 2]),
            ((&[2], &[1, 3]), [2, 3]),
            ((&[1, 2, 5], &[2, 4]), [5, 4]),
        ];

        for ((alice_sizes, bob_sizes), expected) in test_cases {
            assert_eq!(S::fair_candy_swap(alice_sizes.to_vec(), bob_sizes.to_vec()), expected);
        }
    }
}
