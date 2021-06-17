pub mod canonical;
pub mod cheating;
pub mod fast;
pub mod fast_2;
pub mod three_way;
pub mod three_way_fast;

pub trait Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-1, 0, 3, 5, 9, 12] as &[_], 9), 4),
            ((&[-1, 0, 3, 5, 9, 12], 2), -1),
            ((&[], 7), -1),
            ((&[1], 1), 0),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::search(nums.to_vec(), target), expected);
        }
    }
}
