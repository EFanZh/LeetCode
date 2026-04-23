pub mod hash_map;

pub trait Solution {
    fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 1, 3, 2] as &[_], &[5, 3, 4, 6, 2] as &[_]), 14),
            ((&[1, 2, 1, 2] as &[_], &[4, 5, 6, 7] as &[_]), -1),
        ];

        for ((x, y), expected) in test_cases {
            assert_eq!(S::max_sum_distinct_triplet(x.to_vec(), y.to_vec()), expected);
        }
    }
}
