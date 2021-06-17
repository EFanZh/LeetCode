pub mod dynamic_programming;
pub mod mathematical;

pub trait Solution {
    fn optimal_division(nums: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1000, 100, 10, 2] as &[_], "1000/(100/10/2)"),
            (&[2, 3, 4], "2/(3/4)"),
            (&[2], "2"),
            (&[6, 2, 3, 4, 5], "6/(2/3/4/5)"),
            (&[2, 3], "2/3"),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::optimal_division(nums.to_vec()), expected);
        }
    }
}
