pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut num = num;
        let new_length = num.trim_end_matches('0').len();

        num.truncate(new_length);

        num
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_trailing_zeros(num: String) -> String {
        Self::remove_trailing_zeros(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
