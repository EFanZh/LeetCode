pub mod fenwick_tree;
pub mod merge_sort;

pub trait Solution {
    fn count_smaller(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 2, 6, 1] as &[_], &[2, 1, 1, 0] as &[_]), (&[2, 0, 1], &[2, 0, 0])];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_smaller(nums.to_vec()), expected);
        }
    }
}
