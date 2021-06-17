pub mod hash_map;

pub trait Solution {
    fn fraction_to_decimal(numerator: i32, denominator: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 2), "0.5"),
            ((2, 1), "2"),
            ((2, 3), "0.(6)"),
            ((-50, 8), "-6.25"),
            ((7, -12), "-0.58(3)"),
            ((-1, -2_147_483_648), "0.0000000004656612873077392578125"),
            ((-2_147_483_648, -1), "2147483648"),
        ];

        for ((numerator, denominator), expected) in test_cases {
            assert_eq!(S::fraction_to_decimal(numerator, denominator), expected);
        }
    }
}
