pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        let mut states = [(0_u32, 0_u32); 26];

        (0..).zip(s.bytes()).for_each(|(i, c)| {
            let state = &mut states[usize::from(c) - usize::from(b'a')];

            state.0 += 1;
            state.1 = i;
        });

        let max_count = states.iter().fold(0, |max, &(count, _)| max.max(count));
        let mut buffer = [(0, 0); 26];
        let mut length = 0;

        (b'a'..).zip(&states).for_each(|(c, &(count, i))| {
            if count == max_count {
                buffer[length] = (i, c);
                length += 1;
            }
        });

        let buffer = &mut buffer[..length];

        buffer.sort_unstable_by_key(|&(i, _)| i);

        buffer.iter().map(|&(_, c)| char::from(c)).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn last_non_empty_string(s: String) -> String {
        Self::last_non_empty_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
