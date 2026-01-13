pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(start: &[u32], d: u32, interval: u32) -> bool {
        let mut iter = start.iter().copied();

        iter.next().is_none_or(|mut prev| {
            iter.all(|start| {
                let candidate = prev + interval;
                let is_valid = candidate <= start + d;

                if is_valid {
                    prev = start.max(candidate);
                }

                is_valid
            })
        })
    }

    pub fn max_possible_score(start: Vec<i32>, d: i32) -> i32 {
        let mut start = start.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        start.sort_unstable();

        let d = d.cast_unsigned();
        let mut left = 0;
        let mut right = start.last().unwrap() - start.first().unwrap() + d;

        while left < right {
            let left_plus_1 = left + 1;
            let middle = left_plus_1.midpoint(right);

            if Self::check(&start, d, middle) {
                left = middle;
            } else {
                right = middle - 1;
            }
        }

        left.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_possible_score(start: Vec<i32>, d: i32) -> i32 {
        Self::max_possible_score(start, d)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
