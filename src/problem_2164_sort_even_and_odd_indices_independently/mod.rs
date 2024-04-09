pub mod out_of_place_sorting;

pub trait Solution {
    fn sort_even_odd(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 1, 2, 3] as &[_], &[2, 3, 4, 1] as &[_]), (&[2, 1], &[2, 1])];

        for (nums, expected) in test_cases {
            assert_eq!(S::sort_even_odd(nums.to_vec()), expected);
        }
    }
}
