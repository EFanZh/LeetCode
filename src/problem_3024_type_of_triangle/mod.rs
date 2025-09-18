pub mod mathematical;

pub trait Solution {
    fn triangle_type(digits: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 3, 3] as &[_], "equilateral"),
            (&[3, 4, 5], "scalene"),
            (&[9, 4, 9], "isosceles"),
            (&[5, 3, 8], "none"),
        ];

        for (digits, expected) in test_cases {
            assert_eq!(S::triangle_type(digits.to_vec()), expected);
        }
    }
}
