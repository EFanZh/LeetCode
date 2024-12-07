pub mod bit_set;
pub mod hash_set;

pub trait Solution {
    fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    2,
                    &[&[1] as &[_], &[2], &[1, 2]] as &[&[_]],
                    &[&[1, 2] as &[_], &[1, 3], &[2, 3]] as &[&[_]],
                ),
                1,
            ),
            (
                (
                    3,
                    &[&[2], &[1, 3], &[1, 2], &[3]],
                    &[&[1, 4], &[1, 2], &[3, 4], &[2, 3]],
                ),
                2,
            ),
        ];

        for ((n, languages, friendships), expected) in test_cases {
            assert_eq!(
                S::minimum_teachings(
                    n,
                    languages.iter().copied().map(Vec::from).collect(),
                    friendships.iter().copied().map(Vec::from).collect(),
                ),
                expected,
            );
        }
    }
}
