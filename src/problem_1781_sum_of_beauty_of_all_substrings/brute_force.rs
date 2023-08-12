pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let mut iter = s.bytes();
        let mut result = 0;

        while let Some(first) = iter.next() {
            let mut frequencies = [0_u16; 26];

            frequencies[usize::from(first) - usize::from(b'a')] = 1;

            let mut max_frequency = 1;

            for c in iter.clone() {
                let frequency = &mut frequencies[usize::from(c) - usize::from(b'a')];

                *frequency += 1;

                max_frequency = max_frequency.max(*frequency);

                result +=
                    i32::from(max_frequency - frequencies.iter().copied().filter(|&count| count != 0).min().unwrap());
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn beauty_sum(s: String) -> i32 {
        Self::beauty_sum(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
