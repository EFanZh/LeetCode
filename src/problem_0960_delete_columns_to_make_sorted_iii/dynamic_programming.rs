pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let columns = strs.first().map_or(0, String::len);
        let mut cache = Vec::with_capacity(columns);
        let mut max_length = 0;

        for i in 0..columns {
            let length = cache
                .iter()
                .enumerate()
                .filter_map(|(j, &length)| strs.iter().map(String::as_bytes).all(|s| s[j] <= s[i]).then(|| length))
                .max()
                .map_or(1, |length| length + 1);

            cache.push(length);
            max_length = max_length.max(length);
        }

        i32::from(columns as u8 - max_length)
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
