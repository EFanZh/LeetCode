pub mod hash_map;

pub trait Solution {
    fn tuple_same_product(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 4, 6] as &[_], 8), (&[1, 2, 4, 5, 10], 16)];

        for (nums, expected) in test_cases {
            assert_eq!(S::tuple_same_product(nums.to_vec()), expected);
        }
    }
}
