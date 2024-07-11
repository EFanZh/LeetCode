pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn add(lhs: u32, rhs: u32) -> u32 {
        let candidate = lhs + rhs;

        candidate.checked_sub(Self::MODULUS).unwrap_or(candidate)
    }

    fn sub(lhs: u32, rhs: u32) -> u32 {
        lhs.checked_sub(rhs).unwrap_or(lhs + Self::MODULUS - rhs)
    }

    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let roll_max = <[_; 6]>::try_from(roll_max).ok().unwrap().map(|value| value as u32);
        let mut cache = vec![([0_u32; 6], 1_u32); n + 1];
        let mut prev = &cache[0];

        for i in 1..=n {
            let item = {
                let mut choice = 0;
                let mut total = 0;

                let counts = roll_max.map(|limit| {
                    let count = Self::sub(
                        prev.1,
                        cache
                            .get(i.wrapping_sub(limit as usize + 1))
                            .map_or(0, |(prev_counts, prev_sum)| Self::sub(*prev_sum, prev_counts[choice])),
                    );

                    total = Self::add(total, count);

                    choice += 1;

                    count
                });

                (counts, total)
            };

            cache[i] = item;
            prev = &cache[i];
        }

        cache.last().unwrap().1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        Self::die_simulator(n, roll_max)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
