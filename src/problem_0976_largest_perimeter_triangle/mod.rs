pub mod greedy;

pub trait Solution {
    fn largest_perimeter(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 2] as &[_], 5), (&[1, 2, 1], 0), (&[3, 6, 2, 3] as &[_], 8)];

        for (nums, expected) in test_cases {
            assert_eq!(S::largest_perimeter(nums.to_vec()), expected);
        }
    }
}
