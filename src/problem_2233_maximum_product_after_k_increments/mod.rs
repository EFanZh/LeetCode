pub mod quick_select;

pub trait Solution {
    fn maximum_product(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[0, 4] as &[_], 5), 20), ((&[6, 3, 3, 2], 2), 216)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::maximum_product(nums.to_vec(), k), expected);
        }
    }
}
