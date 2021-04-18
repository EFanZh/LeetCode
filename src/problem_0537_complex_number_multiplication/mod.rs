pub mod parsing;

pub trait Solution {
    fn complex_number_multiply(a: String, b: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("1+1i", "1+1i"), "0+2i"),
            (("1+-1i", "1+-1i"), "0+-2i"),
            (("78+-76i", "-86+72i"), "-1236+12152i"),
        ];

        for ((a, b), expected) in test_cases.iter().copied() {
            assert_eq!(S::complex_number_multiply(a.to_string(), b.to_string()), expected);
        }
    }
}
