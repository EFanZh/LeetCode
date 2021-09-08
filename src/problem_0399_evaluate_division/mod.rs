pub mod bfs;

pub trait Solution {
    fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[["a", "b"], ["b", "c"]] as &[_],
                    &[2.0, 3.0] as &[_],
                    &[["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]] as &[_],
                ),
                &[6.0, 0.5, -1.0, 1.0, -1.0] as &[_],
            ),
            (
                (
                    &[["a", "b"], ["b", "c"], ["bc", "cd"]],
                    &[1.5, 2.5, 5.0],
                    &[["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]],
                ),
                &[3.75, 0.4, 5.0, 0.2],
            ),
            (
                (&[["a", "b"]], &[0.5], &[["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]]),
                &[0.5, 2.0, -1.0, -1.0],
            ),
            (
                (
                    &[["a", "b"], ["c", "d"]],
                    &[1.0, 1.0],
                    &[["a", "c"], ["b", "d"], ["b", "a"], ["d", "c"]],
                ),
                &[-1.0, -1.0, 1.0, 1.0],
            ),
            (
                (
                    &[["a", "b"], ["e", "f"], ["b", "e"]],
                    &[3.4, 1.4, 2.3],
                    &[
                        ["b", "a"],
                        ["a", "f"],
                        ["f", "f"],
                        ["e", "e"],
                        ["c", "c"],
                        ["a", "c"],
                        ["f", "e"],
                    ],
                ),
                &[5.0 / 17.0, 10.948, 1.0, 1.0, -1.0, -1.0, 5.0 / 7.0],
            ),
        ];

        for ((equations, values, queries), expected) in test_cases {
            let result = S::calc_equation(
                equations
                    .iter()
                    .map(|&[dividend, divisor]| vec![dividend.to_string(), divisor.to_string()])
                    .collect(),
                values.to_vec(),
                queries
                    .iter()
                    .map(|&[dividend, divisor]| vec![dividend.to_string(), divisor.to_string()])
                    .collect(),
            );

            approx::assert_ulps_eq!(result.as_slice(), expected);
        }
    }
}
