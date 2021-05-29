pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let rule_index = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            _ => 2,
        };

        items.iter().filter(|item| item[rule_index] == rule_value).count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        Self::count_matches(items, rule_key, rule_value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
