pub mod kmp;

pub trait Solution {
    fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[&[1, -1, -1] as &[_], &[3, -2, 0]] as &[&[_]],
                    &[1, -1, 0, 1, -1, -1, 3, -2, 0] as &[_],
                ),
                true,
            ),
            ((&[&[10, -2], &[1, 2, 3, 4]], &[1, 2, 3, 4, 10, -2]), false),
            ((&[&[1, 2, 3], &[3, 4]], &[7, 7, 1, 2, 3, 4, 7, 7]), false),
            ((&[&[21, 22, 21, 22, 21, 30]], &[21, 22, 21, 22, 21, 22, 21, 30]), true),
        ];

        for ((groups, nums), expected) in test_cases {
            assert_eq!(
                S::can_choose(groups.iter().copied().map(Vec::from).collect(), nums.to_vec()),
                expected,
            );
        }
    }
}
