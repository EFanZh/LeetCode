pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(clippy::iter_with_drain, reason = "by design")]
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut code = code;
        let mut electronics = Vec::new();
        let mut grocery = Vec::new();
        let mut pharmacy = Vec::new();
        let mut restaurant = Vec::new();

        code.drain(..)
            .zip(business_line)
            .zip(is_active)
            .for_each(|((code, business_line), is_active)| {
                if is_active {
                    let mut iter = code
                        .bytes()
                        .map(|c| matches!(c, b'a'..=b'z' |b'A'..=b'Z' | b'0'..=b'9' | b'_'));

                    if iter.next() == Some(true) && iter.all(|x| x) {
                        match business_line.as_str() {
                            "electronics" => &mut electronics,
                            "grocery" => &mut grocery,
                            "pharmacy" => &mut pharmacy,
                            "restaurant" => &mut restaurant,
                            _ => return,
                        }
                        .push(code);
                    }
                }
            });

        for bucket in [&mut electronics, &mut grocery, &mut pharmacy, &mut restaurant] {
            bucket.sort_unstable();
        }

        code.extend(electronics.into_iter().chain(grocery).chain(pharmacy).chain(restaurant));

        code
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        Self::validate_coupons(code, business_line, is_active)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
