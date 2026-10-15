pub mod hash_map;

pub trait Solution {
    fn max_product_pair(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], 5), &[[2, 1], [3, 0]] as &[_]),
            ((&[-3, -1, 4, 2], 1), &[[3, 1]]),
            ((&[3, 3, 5], 6), &[[-1, -1]]),
        ];

        for ((nums, target), expected) in test_cases {
            let result = S::max_product_pair(nums.to_vec(), target);

            assert!(expected.iter().any(|expected| result == expected));
        }
    }
}
