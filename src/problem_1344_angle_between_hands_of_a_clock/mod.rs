pub mod mathematical;

pub trait Solution {
    fn angle_clock(hour: i32, minutes: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((12, 30), 165.0), ((3, 30), 75.0), ((3, 15), 7.5), ((1, 57), 76.5)];

        for ((hour, minutes), expected) in test_cases {
            approx::assert_ulps_eq!(S::angle_clock(hour, minutes), expected);
        }
    }
}
