pub mod mathematical;

pub trait Solution {
    fn sum_of_multiples(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(7, 21), (10, 40), (9, 30)];

        for (n, expected) in test_cases {
            assert_eq!(S::sum_of_multiples(n), expected);
        }
    }
}
