pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_mismatched_segments(s1: String, s2: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut start = 0;
        let mut i = 0;

        s1.bytes().zip(s2.bytes()).for_each(|(x, y)| {
            if x == y {
                if start < i {
                    result.push(vec![s1[start..i].to_string(), s2[start..i].to_string()]);
                }

                i += 1;
                start = i;
            } else {
                i += 1;
            }
        });

        if start < i {
            result.push(vec![s1[start..i].to_string(), s2[start..i].to_string()]);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_mismatched_segments(s1: String, s2: String) -> Vec<Vec<String>> {
        Self::find_mismatched_segments(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
