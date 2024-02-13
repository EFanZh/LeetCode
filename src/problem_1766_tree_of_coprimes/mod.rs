pub mod recursive;

pub trait Solution {
    fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[2, 3, 3, 2] as &[_], &[[0, 1], [1, 2], [1, 3]] as &[_]),
                &[-1, 0, 0, 1] as &[_],
            ),
            (
                (
                    &[5, 6, 10, 2, 3, 6, 15],
                    &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                ),
                &[-1, 0, -1, 0, 0, 0, -1],
            ),
        ];

        for ((nums, edges), expected) in test_cases {
            assert_eq!(
                S::get_coprimes(nums.to_vec(), edges.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
