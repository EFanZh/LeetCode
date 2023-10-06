pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let word_length = words.first().map_or(0, String::len);
        let mut cache = vec![0_u32; target.len()].into_boxed_slice();

        for i in 0..word_length {
            let mut counts = [0_u16; 26];

            for word in &words {
                counts[usize::from(word.as_bytes()[i]) - usize::from(b'a')] += 1;
            }

            let mut top_left = 1;

            for (target, c) in cache.iter_mut().zip(target.bytes()) {
                let drop = u64::from(*target);
                let pick = u64::from(top_left) * u64::from(counts[usize::from(c) - usize::from(b'a')]);

                top_left = *target;
                *target = ((drop + pick) % 1_000_000_007) as _;
            }
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_ways(words: Vec<String>, target: String) -> i32 {
        Self::num_ways(words, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
