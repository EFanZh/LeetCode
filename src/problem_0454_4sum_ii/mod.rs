pub mod reduce_to_two_sum;

pub trait Solution {
    fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            (&[1, 2] as &[_], &[-2, -1] as &[_], &[-1, 2] as &[_], &[0, 2] as &[_]),
            2,
        )];

        for ((a, b, c, d), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::four_sum_count(a.to_vec(), b.to_vec(), c.to_vec(), d.to_vec()),
                expected
            );
        }
    }
}
