pub mod sweep_line;

pub trait Solution {
    fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 6], [3, 7], [9, 12], [4, 13]] as &[_], &[2, 3, 7, 11] as &[_]),
                &[1, 2, 2, 2] as &[_],
            ),
            ((&[[1, 10], [3, 3]], &[3, 3, 2]), &[2, 2, 1]),
        ];

        for ((flowers, people), expected) in test_cases {
            assert_eq!(
                S::full_bloom_flowers(flowers.iter().map(Vec::from).collect(), people.to_vec()),
                expected,
            );
        }
    }
}
