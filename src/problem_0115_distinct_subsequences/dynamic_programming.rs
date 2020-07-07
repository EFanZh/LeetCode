pub struct Solution {}

use std::mem;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut cache = vec![0; t.len() + 1];
        let mut temp = vec![0; t.len() + 1];

        cache[0] = 1;
        temp[0] = 1;

        for c_1 in s.into_bytes() {
            for (i, c_2) in t.bytes().enumerate() {
                temp[i + 1] = if c_2 == c_1 {
                    cache[i] + cache[i + 1]
                } else {
                    cache[i + 1]
                };
            }

            mem::swap(&mut cache, &mut temp);
        }

        cache[t.len()]
    }
}

impl super::Solution for Solution {
    fn num_distinct(s: String, t: String) -> i32 {
        Self::num_distinct(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
