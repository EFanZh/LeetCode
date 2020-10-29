pub struct Solution;

impl Solution {
    #[allow(clippy::clippy::wrong_self_convention)]
    pub fn to_lower_case(mut str: String) -> String {
        str.make_ascii_lowercase();

        str
    }
}

impl super::Solution for Solution {
    fn to_lower_case(str: String) -> String {
        Self::to_lower_case(str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
