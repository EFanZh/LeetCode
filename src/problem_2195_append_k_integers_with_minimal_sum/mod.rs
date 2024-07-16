pub mod greedy;

pub trait Solution {
    fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 4, 25, 10, 25] as &[_], 2), 5),
            ((&[5, 6], 6), 25),
            (
                (
                    &[
                        96, 44, 99, 25, 61, 84, 88, 18, 19, 33, 60, 86, 52, 19, 32, 47, 35, 50, 94, 17, 29, 98, 22, 21,
                        72, 100, 40, 84,
                    ],
                    35,
                ),
                794,
            ),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::minimal_k_sum(nums.to_vec(), k), expected);
        }
    }
}
