pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(query: &str, pattern: &str) -> bool {
        let mut iter = query.bytes();

        for p in pattern.bytes() {
            loop {
                if let Some(c) = iter.next() {
                    if c == p {
                        break;
                    }

                    if c.is_ascii_uppercase() {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        iter.all(|c| c.is_ascii_lowercase())
    }

    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        queries.iter().map(|query| Self::check(query, &pattern)).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        Self::camel_match(queries, pattern)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
