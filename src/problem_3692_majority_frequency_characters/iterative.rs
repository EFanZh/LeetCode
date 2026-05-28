pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn majority_frequency_group(s: String) -> String {
        let mut counts = [0_u8; 26];

        for c in s.into_bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut groups = [const { String::new() }; 100];

        (b'a'..).zip(&counts).for_each(|(c, &count)| {
            if let Some(group) = groups.get_mut(usize::from(count).wrapping_sub(1)) {
                group.push(char::from(c));
            }
        });

        groups.into_iter().fold(
            String::new(),
            |result, group| {
                if group.len() < result.len() { result } else { group }
            },
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn majority_frequency_group(s: String) -> String {
        Self::majority_frequency_group(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
