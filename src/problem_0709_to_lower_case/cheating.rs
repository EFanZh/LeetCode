pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut str = str;

        str.make_ascii_lowercase();

        str
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
