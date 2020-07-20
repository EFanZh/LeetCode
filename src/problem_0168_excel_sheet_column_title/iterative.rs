pub struct Solution {}

impl Solution {
    pub fn convert_to_title(mut n: i32) -> String {
        let mut result = Vec::new();

        while n != 0 {
            n -= 1;
            result.push(b'A' + (n % 26) as u8);
            n /= 26;
        }

        result.reverse();

        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn convert_to_title(n: i32) -> String {
        Self::convert_to_title(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
