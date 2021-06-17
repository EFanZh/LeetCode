pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn count_bits(num: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(2, &[0, 1, 1] as &[_]), (5, &[0, 1, 1, 2, 1, 2])];

        for (num, expected) in test_cases {
            assert_eq!(S::count_bits(num), expected);
        }
    }
}
