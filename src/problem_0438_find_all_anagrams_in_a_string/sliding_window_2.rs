pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = Vec::new();

        if p.len() <= s.len() {
            let mut counts = [0; 26];

            for c in p.bytes() {
                counts[usize::from(c - b'a')] += 1;
            }

            let mut i = 0;
            let (first, rest) = s.as_bytes().split_at(p.len());

            for c in first {
                counts[usize::from(c - b'a')] -= 1;
            }

            if counts.iter().all(|&x| x == 0) {
                result.push(i);
            }

            for (c_1, c_2) in s.bytes().zip(rest) {
                counts[usize::from(c_1 - b'a')] += 1;
                counts[usize::from(c_2 - b'a')] -= 1;
                i += 1;

                if counts.iter().all(|&x| x == 0) {
                    result.push(i);
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_anagrams(s: String, p: String) -> Vec<i32> {
        Self::find_anagrams(s, p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
