pub mod binary_heap;

pub trait Solution {
    fn k_sum(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 4, -2] as &[_], 5), 2),
            ((&[1, -2, 3, 4, -10, 12], 16), 10),
            ((&[-1, 1], 1), 1),
            ((&[492_634_335, 899_178_915, 230_945_927], 2), 1_391_813_250),
            ((&[-530_219_056, 353_285_209, 493_533_664], 6), -36_685_392),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::k_sum(nums.to_vec(), k), expected);
        }
    }
}
