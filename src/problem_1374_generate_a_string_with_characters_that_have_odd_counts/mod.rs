pub mod iterative;

pub trait Solution {
    fn generate_the_string(n: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [1, 2, 3, 4, 5, 6, 7];

        for n in test_cases {
            let mut counts = [0; 26];

            for c in S::generate_the_string(n).into_bytes() {
                counts[usize::from(c) - usize::from(b'a')] += 1;
            }

            for count in counts {
                assert!(count == 0 || count % 2 == 1);
            }
        }
    }
}
