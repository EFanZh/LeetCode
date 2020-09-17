pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() == t.len() {
            let mut counts = [0; 26];

            for c in s.bytes() {
                counts[usize::from(c - b'a')] += 1;
            }

            for c in t.bytes() {
                let count = &mut counts[usize::from(c - b'a')];

                if *count == 0 {
                    return false;
                } else {
                    *count -= 1;
                }
            }

            counts.iter().copied().all(|x| x == 0)
        } else {
            false
        }
    }
}

impl super::Solution for Solution {
    fn is_anagram(s: String, t: String) -> bool {
        Self::is_anagram(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
