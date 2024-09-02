pub mod brute_force;

pub trait Solution {
    fn minimize_result(expression: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("247+38", 170), ("12+34", 20), ("999+999", 1998)];

        for (expression, expected) in test_cases {
            let result = S::minimize_result(expression.to_string());
            let result_middle = result.find('+').unwrap();
            let left = &result[..result_middle];
            let left_split = left.find('(').unwrap();
            let left_left = &left[..left_split];
            let left_right = &left[left_split + 1..];
            let right = &result[result_middle + 1..];
            let right_split = right.find(')').unwrap();
            let right_left = &right[..right_split];
            let right_right = &right[right_split + 1..];

            assert_eq!(format!("{left_left}{left_right}+{right_left}{right_right}"), expression);

            let parse = |s: &str| s.parse::<u32>().unwrap();
            let parse_outer = |s: &str| if s.is_empty() { 1_u32 } else { s.parse().unwrap() };

            assert_eq!(
                parse_outer(left_left) * (parse(left_right) + parse(right_left)) * parse_outer(right_right),
                expected,
            );
        }
    }
}
