pub mod binary_search;

pub trait Solution {
    fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 5, 2, 1] as &[_], &[3, 10, 21] as &[_]), &[2, 3, 4] as &[_]),
            ((&[2, 3, 4, 5], &[1]), &[0]),
        ];

        for ((nums, queries), expected) in test_cases {
            assert_eq!(S::answer_queries(nums.to_vec(), queries.to_vec()), expected);
        }
    }
}
