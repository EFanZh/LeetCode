pub mod recursive;

pub trait Solution {
    fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("e + 8 - a + 5", &["e"] as &[_], &[1] as &[_]), &["-1*a", "14"] as &[_]),
            (
                ("e - 8 + temperature - pressure", &["e", "temperature"], &[1, 12]),
                &["-1*pressure", "5"],
            ),
            (("(e + 8) * (e - 8)", &[], &[]), &["1*e*e", "-64"]),
            (("a * b * c + b * a * c * 4", &[], &[]), &["5*a*b*c"]),
            (
                (
                    "((a - b) * (b - c) + (c - a)) * ((a - b) + (b - c) * (c - a))",
                    &[],
                    &[],
                ),
                &[
                    "-1*a*a*b*b",
                    "2*a*a*b*c",
                    "-1*a*a*c*c",
                    "1*a*b*b*b",
                    "-1*a*b*b*c",
                    "-1*a*b*c*c",
                    "1*a*c*c*c",
                    "-1*b*b*b*c",
                    "2*b*b*c*c",
                    "-1*b*c*c*c",
                    "2*a*a*b",
                    "-2*a*a*c",
                    "-2*a*b*b",
                    "2*a*c*c",
                    "1*b*b*b",
                    "-1*b*b*c",
                    "1*b*c*c",
                    "-1*c*c*c",
                    "-1*a*a",
                    "1*a*b",
                    "1*a*c",
                    "-1*b*c",
                ],
            ),
        ];

        for ((expression, evalvars, evalints), expected) in test_cases {
            assert_eq!(
                S::basic_calculator_iv(
                    expression.to_string(),
                    evalvars.iter().copied().map(str::to_string).collect(),
                    evalints.to_vec()
                ),
                expected
            );
        }
    }
}
