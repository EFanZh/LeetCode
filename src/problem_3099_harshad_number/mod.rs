pub mod binary_search;

pub trait Solution {
    fn sum_of_the_digits_of_harshad_number(x: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(18, 9)];

        for (x, expected) in test_cases {
            assert_eq!(S::sum_of_the_digits_of_harshad_number(x), expected);
        }
    }
}
