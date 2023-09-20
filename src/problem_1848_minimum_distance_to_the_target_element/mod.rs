pub mod iterative;

pub trait Solution {
    fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 5, 3), 1),
            ((&[1], 1, 0), 0),
            ((&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0), 0),
            ((&[5, 3, 6], 5, 2), 2),
        ];

        for ((nums, target, start), expected) in test_cases {
            assert_eq!(S::get_min_distance(nums.to_vec(), target, start), expected);
        }
    }
}
