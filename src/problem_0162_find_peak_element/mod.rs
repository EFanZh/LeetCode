pub mod binary_search;

pub trait Solution {
    fn find_peak_element(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 1] as &[_], &[2] as &[_]), (&[1, 2, 1, 3, 5, 6, 4], &[1, 5])];

        for (nums, expected) in test_cases {
            assert!(expected.contains(&S::find_peak_element(nums.to_vec())));
        }
    }
}
