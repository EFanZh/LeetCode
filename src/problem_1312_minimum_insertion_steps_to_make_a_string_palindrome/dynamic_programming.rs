pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.into_bytes();
        let n = s.len();
        let mut buffer = vec![0_u32; n * 3];
        let (mut cache_1, rest) = buffer.split_at_mut(n);
        let (mut cache_2, mut temp) = rest.split_at_mut(n);

        for length in 2..=n {
            for (start, (target, is_same)) in temp
                .iter_mut()
                .zip(s.iter().zip(&s[length - 1..]).map(|(lhs, rhs)| lhs == rhs))
                .enumerate()
            {
                *target = if is_same {
                    cache_1[start + 1]
                } else {
                    cache_2[start].min(cache_2[start + 1]) + 1
                };
            }

            let swap = cache_1;

            cache_1 = cache_2;
            cache_2 = temp;
            temp = swap;
        }

        cache_2[0] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_insertions(s: String) -> i32 {
        Self::min_insertions(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
