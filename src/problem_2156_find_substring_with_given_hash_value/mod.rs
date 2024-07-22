pub mod iterative;

pub trait Solution {
    fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("leetcode", 7, 20, 2, 0), "ee"), (("fbxzaad", 31, 100, 3, 32), "fbx")];

        for ((s, power, modulo, k, hash_value), expected) in test_cases {
            assert_eq!(S::sub_str_hash(s.to_string(), power, modulo, k, hash_value), expected);
        }
    }
}
