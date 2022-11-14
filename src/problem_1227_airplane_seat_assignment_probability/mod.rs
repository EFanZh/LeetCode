pub mod mathematical;

pub trait Solution {
    fn nth_person_gets_nth_seat(n: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1.0), (2, 0.5)];

        for (n, expected) in test_cases {
            approx::assert_ulps_eq!(S::nth_person_gets_nth_seat(n), expected);
        }
    }
}
