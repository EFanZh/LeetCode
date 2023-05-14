pub mod fenwick_tree;

pub trait Solution {
    fn min_integer(num: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("4321", 4), "1342"),
            (("100", 1), "010"),
            (("36789", 1000), "36789"),
            (("22", 22), "22"),
        ];

        for ((num, k), expected) in test_cases {
            assert_eq!(S::min_integer(num.to_string(), k), expected);
        }
    }
}
