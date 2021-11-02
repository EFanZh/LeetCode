pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_number(s: &str, mut callback: impl FnMut((&str, &str))) {
        if s.len() == 1 {
            callback((s, ""));
        } else if let Some(suffix) = s.strip_prefix('0') {
            if !s.ends_with('0') {
                callback(("0", suffix));
            }
        } else if s.ends_with('0') {
            callback((s, ""));
        } else {
            for i in 1..=s.len() {
                callback(s.split_at(i));
            }
        }
    }

    fn measure_number_length(parts: (&str, &str)) -> usize {
        let integer_length = parts.0.len();

        if parts.1.is_empty() {
            integer_length
        } else {
            integer_length + 1 + parts.1.len()
        }
    }

    fn write_number(target: &mut String, parts: (&str, &str)) {
        target.push_str(parts.0);

        if !parts.1.is_empty() {
            target.push('.');
            target.push_str(parts.1);
        }
    }

    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let s = &s[1..s.len() - 1];

        for i in 1..s.len() {
            let (left, right) = s.split_at(i);

            Self::parse_number(left, |x| {
                Self::parse_number(right, |y| {
                    let mut item =
                        String::with_capacity(Self::measure_number_length(x) + Self::measure_number_length(y) + 4);

                    item.push('(');
                    Self::write_number(&mut item, x);
                    item.push_str(", ");
                    Self::write_number(&mut item, y);
                    item.push(')');

                    result.push(item);
                });
            });
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ambiguous_coordinates(s: String) -> Vec<String> {
        Self::ambiguous_coordinates(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
