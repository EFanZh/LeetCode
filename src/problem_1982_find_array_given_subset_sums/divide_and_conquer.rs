pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn recover_array(n: i32, sums: Vec<i32>) -> Vec<i32> {
        let n = n as u8;
        let mut sums = sums;

        sums.sort_unstable();

        let mut zero = 0;

        (0..n)
            .map(|_| {
                let [min_1, min_2] = sums[..2].try_into().ok().unwrap();
                let mut diff = min_2 - min_1;

                // +---------------+-------------------+---------+-----------+
                // | fully matched | partially matched | checked | unchecked |
                // +---------------+-------------------+---------+-----------+

                let half = sums.len() / 2;
                let mut fully_matched = 1;
                let mut i = 2;

                for all_matched in 1..half {
                    while fully_matched < all_matched && sums[i] - sums[fully_matched] == diff {
                        fully_matched += 1;
                        i += 1;
                    }

                    sums[all_matched] = sums[i];
                    i += 1;
                }

                sums.truncate(half);

                if sums.binary_search(&zero).is_err() {
                    zero -= diff;
                    diff = -diff;
                }

                diff
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn recover_array(n: i32, sums: Vec<i32>) -> Vec<i32> {
        Self::recover_array(n, sums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
