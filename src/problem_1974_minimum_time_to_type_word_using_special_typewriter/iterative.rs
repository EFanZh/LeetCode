pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut current = b'a';
        let mut result = word.len() as u32;

        for c in word.bytes() {
            let (min, max) = if c < current { (c, current) } else { (current, c) };

            result += u32::from((max - min).min(min + 26 - max));

            current = c;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_time_to_type(word: String) -> i32 {
        Self::min_time_to_type(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
