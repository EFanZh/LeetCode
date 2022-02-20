pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(lhs: u32, rhs: u32) -> u32 {
        let candidate = lhs + rhs;

        candidate.checked_sub(1_000_000_007).unwrap_or(candidate)
    }

    pub fn knight_dialer(n: i32) -> i32 {
        let n = n as u32;

        // 0 1 0
        // 2   2
        // 0 1 0
        //   3

        if n == 1 {
            10
        } else {
            let mut cache = (1, 1, 1, 1);

            for _ in 1..n {
                cache = (
                    Self::add(cache.1, cache.2),
                    Self::add(cache.0, cache.0),
                    Self::add(Self::add(cache.0, cache.0), cache.3),
                    Self::add(cache.2, cache.2),
                );
            }

            [
                cache.0, cache.0, cache.0, cache.0, cache.1, cache.1, cache.2, cache.2, cache.3,
            ]
            .iter()
            .copied()
            .fold(0, Self::add) as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn knight_dialer(n: i32) -> i32 {
        Self::knight_dialer(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
