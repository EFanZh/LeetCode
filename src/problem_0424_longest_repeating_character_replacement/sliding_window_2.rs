pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let k = k as usize;
        let mut counts = [0; 26];
        let mut start = 0;
        let mut end = 0;
        let mut max_count = 0;

        while let Some(&c) = s.get(end) {
            let count = &mut counts[usize::from(c - b'A')];

            *count += 1;
            max_count = max_count.max(*count);

            if max_count + k <= end - start {
                counts[usize::from(s[start] - b'A')] -= 1;
                start += 1;
            }

            end += 1;
        }

        (end - start) as _
    }
}

impl super::Solution for Solution {
    fn character_replacement(s: String, k: i32) -> i32 {
        Self::character_replacement(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
