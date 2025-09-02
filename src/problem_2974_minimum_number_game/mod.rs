pub mod sorting;

pub trait Solution {
    fn number_game(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 4, 2, 3] as &[_], &[3, 2, 5, 4] as &[_]), (&[2, 5], &[5, 2])];

        for (nums, expected) in test_cases {
            assert_eq!(S::number_game(nums.to_vec()), expected);
        }
    }
}
