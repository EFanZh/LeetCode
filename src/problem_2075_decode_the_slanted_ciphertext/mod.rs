pub mod iterative;

pub trait Solution {
    fn decode_ciphertext(encoded_text: String, rows: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ch   ie   pr", 3), "cipher"),
            (("iveo    eed   l te   olc", 4), "i love leetcode"),
            (("coding", 1), "coding"),
            (("", 5), ""),
        ];

        for ((encoded_text, rows), expected) in test_cases {
            assert_eq!(S::decode_ciphertext(encoded_text.to_string(), rows), expected);
        }
    }
}
