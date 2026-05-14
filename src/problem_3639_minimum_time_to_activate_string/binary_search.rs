pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn check(s: &[u8], k: u32) -> bool {
        let mut prev = 0;

        (1..)
            .zip(s)
            .try_fold(0_u32, |result, (i, &c)| {
                if c == 0 {
                    prev = i;
                }

                result.checked_add(prev)
            })
            .is_some_and(|result| result < k)
    }

    pub fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
        let mut s = s.into_bytes();
        let k = k.cast_unsigned();
        let mut buffer = Vec::new();
        let n = s.len();

        if n * (n + 1) / 2 < k as usize {
            return -1;
        }

        let mut left = 1;
        let mut right = s.len();

        while left < right {
            let middle = left.midpoint(right);
            let indices = &order[..middle];

            buffer.extend(
                indices
                    .iter()
                    .map(|index| mem::take(&mut s[index.cast_unsigned() as usize])),
            );

            let check_result = Self::check(&s, k);

            buffer
                .iter()
                .zip(&order)
                .for_each(|(&c, index)| s[index.cast_unsigned() as usize] = c);

            buffer.clear();

            if check_result {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as i32 - 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
        Self::min_time(s, order, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
