pub mod greedy;

pub trait Solution {
    fn str_without3a3b(a: i32, b: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 2), (4, 1), (1, 1)];

        for (a, b) in test_cases {
            let result = S::str_without3a3b(a, b);

            assert_eq!(result.len(), (a + b) as _);
            assert!(!result.contains("aaa"));
            assert!(!result.contains("bbb"));

            assert_eq!(result.bytes().filter(|&c| c == b'a').count(), a as _);
            assert_eq!(result.bytes().filter(|&c| c == b'b').count(), b as _);
        }
    }
}
