pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cuts = cuts;
        let cuts = cuts.as_mut_slice();

        cuts.sort_unstable();

        let total_sticks = cuts.len() + 1;
        let mut cache = vec![0; total_sticks * total_sticks].into_boxed_slice();

        for sticks in 2..=total_sticks {
            for start in 0..=total_sticks - sticks {
                let end = start + sticks;
                let mut cost = i32::MAX;

                for cut in start + 1..end {
                    cost = cost.min(
                        cache[total_sticks * (cut - start - 1) + start] + cache[total_sticks * (end - cut - 1) + cut],
                    );
                }

                let left = cuts.get(start.wrapping_sub(1)).copied().unwrap_or(0);
                let right = cuts.get(end - 1).copied().unwrap_or(n);

                cost += right - left;

                cache[total_sticks * (sticks - 1) + start] = cost;
            }
        }

        cache[total_sticks * (total_sticks - 1)]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        Self::min_cost(n, cuts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
