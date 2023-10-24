pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut num = num;
        let length = num.bytes().rposition(|c| c & 1 != 0).map_or(0, |i| i + 1);

        num.truncate(length);

        num
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_odd_number(num: String) -> String {
        Self::largest_odd_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
