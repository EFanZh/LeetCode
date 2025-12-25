pub mod mathematical;

pub trait Solution {
    fn non_special_count(l: i32, r: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 7), 3), ((4, 16), 11), ((1, 2), 2)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::non_special_count(n, k), expected);
        }
    }
}
