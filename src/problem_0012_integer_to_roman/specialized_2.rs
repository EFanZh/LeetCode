pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        const C: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        const X: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        const I: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

        let mut result = String::new();

        for _ in 0..num / 1000 {
            result.push('M');
        }

        result.push_str(C[((num % 1000) / 100) as usize]);
        result.push_str(X[((num % 100) / 10) as usize]);
        result.push_str(I[(num % 10) as usize]);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn int_to_roman(num: i32) -> String {
        Self::int_to_roman(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
