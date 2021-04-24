pub mod group_by_thousands;

pub trait Solution {
    fn number_to_words(num: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (123, "One Hundred Twenty Three"),
            (12345, "Twelve Thousand Three Hundred Forty Five"),
            (1_234_567, "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"),
            (1_234_567_891, "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"),
            (0, "Zero"),
            (1000, "One Thousand")
        ];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::number_to_words(num), expected);
        }
    }
}
