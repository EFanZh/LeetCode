pub mod iterative;

pub trait Solution {
    fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[9, 12, 5, 10, 14, 3, 10] as &[_], 10),
                &[9, 5, 3, 10, 10, 12, 14] as &[_],
            ),
            ((&[-3, 4, 3, 2], 2), &[-3, 2, 4, 3]),
        ];

        for ((nums, pivot), expected) in test_cases {
            assert_eq!(S::pivot_array(nums.to_vec(), pivot), expected);
        }
    }
}
