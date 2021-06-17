pub mod sliding_window;

pub trait Solution {
    fn find_max_average(nums: Vec<i32>, k: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 12, -5, -6, 50, 3] as &[_], 4), 12.75), ((&[5], 1), 5.0)];

        for ((nums, k), expected) in test_cases {
            approx::assert_relative_eq!(S::find_max_average(nums.to_vec(), k), expected);
        }
    }
}
