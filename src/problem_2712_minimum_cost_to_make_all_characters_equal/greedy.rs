pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let n = s.len();
        let mut iter = s.bytes().enumerate();

        iter.next().map_or(0, |(_, mut prev)| {
            let mut result = 0_u64;

            for (i, c) in iter {
                if c != prev {
                    result += i.min(n - i) as u64;
                    prev = c;
                }
            }

            result
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_cost(s: String) -> i64 {
        Self::minimum_cost(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
