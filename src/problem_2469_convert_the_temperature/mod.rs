pub mod mathematical;

pub trait Solution {
    fn convert_temperature(celsius: f64) -> Vec<f64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(36.5, [309.65, 97.7]), (122.11, [395.26, 251.798])];

        for (celsius, expected) in test_cases {
            assert_eq!(S::convert_temperature(celsius), expected);
        }
    }
}
