pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = if word2.len() < word1.len() {
            (word2.as_str(), word1.as_str())
        } else {
            (word1.as_str(), word2.as_str())
        };

        let mut cache = (1..=word1.len() as u16).collect::<Box<_>>();

        for (mut top_left, c1) in (0..).zip(word2.bytes()) {
            let mut left = top_left + 1;

            for (target, c2) in cache.iter_mut().zip(word1.bytes()) {
                let distance = if c1 == c2 {
                    top_left
                } else {
                    top_left.min(*target).min(left) + 1
                };

                top_left = *target;
                left = distance;
                *target = distance;
            }
        }

        cache.last().map_or(word2.len() as _, |&last| i32::from(last))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_distance(word1: String, word2: String) -> i32 {
        Self::min_distance(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
