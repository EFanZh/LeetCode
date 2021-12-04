pub mod brute_force;

pub trait Solution {
    fn split_into_fibonacci(num: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("123456579", &[123, 456, 579] as &[_]),
            ("11235813", &[1, 1, 2, 3, 5, 8, 13]),
            ("112358130", &[]),
            ("0123", &[]),
            ("1101111", &[11, 0, 11, 11]),
            ("0000", &[0, 0, 0, 0]),
            (
                "539834657215398346785398346991079669377161950407626991734534318677529701785098211336528511",
                &[],
            ),
            ("214748364721474836422147483641", &[]),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::split_into_fibonacci(num.to_string()), expected);
        }
    }
}
