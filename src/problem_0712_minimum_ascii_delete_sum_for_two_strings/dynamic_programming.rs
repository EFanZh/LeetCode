pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let (s1, s2) = if s2.len() < s1.len() { (s2, s1) } else { (s1, s2) };
        let (s1, s2) = (s1.into_bytes(), s2.into_bytes());

        let mut cache = s1
            .iter()
            .scan(0, |sum, &x| {
                *sum += i32::from(x);

                Some(*sum)
            })
            .collect::<Vec<_>>();

        let mut top_left = 0;

        for &c2 in &s2 {
            let mut left = top_left + i32::from(c2);
            let next_top_left = left;

            for (&c1, slot) in s1.iter().zip(&mut cache) {
                let top = *slot;

                *slot = if c1 == c2 {
                    top_left
                } else {
                    (i32::from(c1) + left).min(i32::from(c2) + top)
                };

                top_left = top;
                left = *slot;
            }

            top_left = next_top_left;
        }

        *cache.last().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        Self::minimum_delete_sum(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
