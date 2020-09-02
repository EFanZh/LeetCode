pub mod sliding_window;
pub mod tail_recursive_sliding_window;

pub trait Solution {
    fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((7, &[2, 3, 1, 2, 4, 3] as &[_]), 2)];

        for ((s, nums), expected) in test_cases.iter().copied() {
            assert_eq!(S::min_sub_array_len(s, nums.to_vec()), expected);
        }
    }
}
