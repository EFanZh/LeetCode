pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_vowel(c: u8) -> bool {
        matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
    }

    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count = 0;

        let prefix_sums = words
            .into_iter()
            .map(|word| {
                count += u32::from(
                    word.as_bytes().first().copied().is_some_and(Self::is_vowel)
                        && word.as_bytes().last().copied().is_some_and(Self::is_vowel),
                );

                count
            })
            .collect::<Box<_>>();

        queries
            .into_iter()
            .map(|query| {
                let [left, right] = query.try_into().ok().unwrap();

                (prefix_sums[right as u32 as usize]
                    - prefix_sums
                        .get((left as u32 as usize).wrapping_sub(1))
                        .copied()
                        .unwrap_or(0)) as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::vowel_strings(words, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
