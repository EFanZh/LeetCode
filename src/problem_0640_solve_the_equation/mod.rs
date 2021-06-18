pub mod iterative;

pub trait Solution {
    fn solve_equation(equation: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("x+5-3+x=6+x-2", "x=2"),
            ("x=x", "Infinite solutions"),
            ("2x=x", "x=0"),
            ("2x+3x-6x=x+2", "x=-1"),
            ("x=x+2", "No solution"),
            ("3x=33+22+11", "x=22"),
            ("-x=-1", "x=1"),
        ];

        for (equation, expected) in test_cases {
            assert_eq!(S::solve_equation(equation.to_string()), expected);
        }
    }
}
