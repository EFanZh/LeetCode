pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = Vec::new();

        if let Some((first, rest)) = strs.split_first() {
            for (i, c) in first.bytes().enumerate() {
                if rest.iter().all(|s| s.as_bytes().get(i).copied() == Some(c)) {
                    result.push(c);
                } else {
                    break;
                }
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        Self::longest_common_prefix(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
