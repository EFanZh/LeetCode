pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut counts = [0_u8; 26];

        for c in word.into_bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        counts.sort_unstable();

        let start = counts.iter().rposition(|&count| count == 0).map_or(0, |i| i + 1);

        #[expect(clippy::unnecessary_map_or, reason = "compatibility")]
        if counts[start] == 1
            && counts.get(start + 1).map_or(true, |&first_count| {
                counts[start + 2..].iter().all(|&count| count == first_count)
            })
        {
            return true;
        }

        let target = *counts.last().unwrap() - 1;

        counts[start..25].iter().all(|&count| count == target)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn equal_frequency(word: String) -> bool {
        Self::equal_frequency(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
