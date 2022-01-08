pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        const MODULUS: u32 = 1_000_000_007;

        let n = n as usize;
        let min_profit = min_profit as usize;
        let columns = n + 1;
        let mut cache = vec![0_u32; columns * (min_profit + 1)];

        for target in &mut cache[..columns] {
            *target = 1;
        }

        for (&scheme_profit, &scheme_members) in profit.iter().zip(&group) {
            let scheme_profit = scheme_profit as usize;
            let scheme_members = scheme_members as usize;

            for profit in (0..=min_profit).rev() {
                for members in (scheme_members..=n).rev() {
                    let extra_count = cache[columns * profit.saturating_sub(scheme_profit) + members - scheme_members];
                    let count = &mut cache[columns * profit + members];

                    *count += extra_count;

                    if let Some(normalized_count) = count.checked_sub(MODULUS) {
                        *count = normalized_count;
                    }
                }
            }
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        Self::profitable_schemes(n, min_profit, group, profit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
