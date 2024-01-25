pub mod sliding_window;

pub trait Solution {
    fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[7, 4, 3, 9, 1, 8, 5, 2, 6] as &[_], 3),
                &[-1, -1, -1, 5, 4, 4, -1, -1, -1] as &[_],
            ),
            ((&[100_000], 0), &[100_000]),
            ((&[8], 100_000), &[-1]),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::get_averages(nums.to_vec(), k), expected);
        }
    }
}
