pub struct Solution {}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        const C: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        const X: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        const I: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

        let mut result = String::new();

        for _ in 0..num / 1000 {
            result.push_str("M");
        }

        num %= 1000;
        result.push_str(C[(num / 100) as usize]);
        num %= 100;
        result.push_str(X[(num / 10) as usize]);
        num %= 10;
        result.push_str(I[num as usize]);

        result
    }
}

impl super::Solution for Solution {
    fn int_to_roman(num: i32) -> String {
        Self::int_to_roman(num)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
