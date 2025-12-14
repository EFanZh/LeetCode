pub mod rotation;

pub trait Solution {
    fn get_encrypted_string(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("dart", 3), "tdar"), (("aaa", 1), "aaa")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::get_encrypted_string(s.to_string(), k), expected);
        }
    }
}
