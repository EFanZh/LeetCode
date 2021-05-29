pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_indices = [0; 26];

        for (i, c) in (0..).zip(s.bytes()) {
            last_indices[usize::from(c - b'a')] = i;
        }

        let mut result = Vec::with_capacity(26);
        let (&first, rest) = s.as_bytes().split_first().unwrap();
        let mut start = 0;
        let mut end = last_indices[usize::from(first - b'a')];

        for (i, &c) in (1..).zip(rest) {
            if i == end + 1 {
                result.push(end - start + 1);
                start = i;
                end = last_indices[usize::from(c - b'a')];
            } else {
                end = end.max(last_indices[usize::from(c - b'a')]);
            }
        }

        result.push(end - start + 1);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn partition_labels(s: String) -> Vec<i32> {
        Self::partition_labels(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
