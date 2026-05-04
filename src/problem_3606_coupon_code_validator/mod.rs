pub mod bucket_sort;

pub trait Solution {
    fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["SAVE20", "", "PHARMA5", "SAVE@20"] as &[_],
                    &["restaurant", "grocery", "pharmacy", "restaurant"] as &[_],
                    &[true, true, true, true] as &[_],
                ),
                &["PHARMA5", "SAVE20"] as &[_],
            ),
            (
                (
                    &["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"],
                    &["grocery", "electronics", "invalid"],
                    &[false, true, true],
                ),
                &["ELECTRONICS_50"],
            ),
            (
                (
                    &["SAVE20", "", "PHARMA5", "SAVE@20"],
                    &["restaurant", "grocery", "pharmacy", "restaurant"],
                    &[true, true, true, true],
                ),
                &["PHARMA5", "SAVE20"],
            ),
        ];

        for ((code, business_line, is_active), expected) in test_cases {
            assert_eq!(
                S::validate_coupons(
                    code.iter().copied().map(str::to_string).collect(),
                    business_line.iter().copied().map(str::to_string).collect(),
                    is_active.to_vec(),
                ),
                expected,
            );
        }
    }
}
