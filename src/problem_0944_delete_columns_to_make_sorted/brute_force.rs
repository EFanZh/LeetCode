pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let columns = strs.first().map_or(0, String::len);
        let mut result = 0;

        for i in 0..columns {
            let mut prev = 0;

            if strs.iter().any(|s| {
                let c = s.as_bytes()[i];
                let result = c < prev;

                prev = c;

                result
            }) {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_deletion_size(strs: Vec<String>) -> i32 {
        Self::min_deletion_size(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
