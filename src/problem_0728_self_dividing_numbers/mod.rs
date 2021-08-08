pub mod brute_force;
pub mod brute_force_2;

pub trait Solution {
    fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 22), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22] as &[_]),
            ((47, 85), &[48, 55, 66, 77]),
        ];

        for ((left, right), expected) in test_cases {
            assert_eq!(S::self_dividing_numbers(left, right), expected);
        }
    }
}
