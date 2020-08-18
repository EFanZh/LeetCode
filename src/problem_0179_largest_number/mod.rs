pub mod sorting_1;
pub mod sorting_2;

pub trait Solution {
    fn largest_number(nums: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[10, 2] as &[_], "210"),
            (&[3, 30, 34, 5, 9], "9534330"),
            (&[0, 0], "0"),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::largest_number(nums.to_vec()), expected);
        }
    }
}
