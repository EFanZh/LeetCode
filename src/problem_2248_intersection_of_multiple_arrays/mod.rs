pub mod iterative;

pub trait Solution {
    fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[3, 1, 2, 4, 5] as &[_], &[1, 2, 3, 4], &[3, 4, 5, 6]] as &[&[_]],
                &[3, 4] as &[_],
            ),
            (&[&[1, 2, 3], &[4, 5, 6]], &[]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::intersection(nums.iter().copied().map(Vec::from).collect()), expected);
        }
    }
}
