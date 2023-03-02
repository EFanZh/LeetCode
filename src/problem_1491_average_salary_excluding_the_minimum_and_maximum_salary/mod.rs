pub mod iterative;

pub trait Solution {
    fn average(salary: Vec<i32>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4000, 3000, 1000, 2000] as &[_], 2500.0),
            (&[1000, 2000, 3000], 2000.0),
        ];

        for (salary, expected) in test_cases {
            approx::assert_ulps_eq!(S::average(salary.to_vec()), expected);
        }
    }
}
