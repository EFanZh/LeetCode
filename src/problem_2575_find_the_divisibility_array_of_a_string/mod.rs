pub mod modular_arithmetic;

pub trait Solution {
    fn divisibility_array(word: String, m: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("998244353", 3), &[1, 1, 0, 0, 0, 1, 1, 0, 0] as &[_]),
            (("1010", 10), &[0, 1, 0, 1]),
        ];

        for ((word, m), expected) in test_cases {
            assert_eq!(S::divisibility_array(word.to_string(), m), expected);
        }
    }
}
