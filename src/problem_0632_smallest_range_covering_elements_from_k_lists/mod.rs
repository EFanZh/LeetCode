pub mod binary_heap;
pub mod sliding_window;

pub trait Solution {
    fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[4, 10, 15, 24, 26] as &[_], &[0, 9, 12, 20], &[5, 18, 22, 30]] as &[&[_]],
                [20, 24],
            ),
            (&[&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]], [1, 1]),
            (&[&[10, 10], &[11, 11]], [10, 11]),
            (&[&[10], &[11]], [10, 11]),
            (&[&[1], &[2], &[3], &[4], &[5], &[6], &[7]], [1, 7]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(
                S::smallest_range(nums.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
