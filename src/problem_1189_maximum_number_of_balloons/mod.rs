pub mod count_letters;

pub trait Solution {
    fn max_number_of_balloons(text: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("nlaebolko", 1), ("loonbalxballpoon", 2), ("leetcode", 0)];

        for (text, expected) in test_cases {
            assert_eq!(S::max_number_of_balloons(text.to_string()), expected);
        }
    }
}
