pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    pub fn exp_mod(mut base: u64, mut exponent: u32) -> u64 {
        let mut result = 1;

        loop {
            if exponent & 1 != 0 {
                result = (result * base) % Self::MODULUS;
            }

            exponent >>= 1;

            if exponent == 0 {
                break;
            }

            base = (base * base) % Self::MODULUS;
        }

        result
    }

    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let mut events = ranges
            .into_iter()
            .flat_map(|range| {
                let [x, y] = range.try_into().ok().unwrap();

                [(x as u32) << 2, ((y as u32) << 2) + 2]
            })
            .collect::<Box<_>>();

        events.sort_unstable();

        let mut count = 0;
        let mut groups = 0;

        for event in events {
            let action = event & 3;

            groups += u32::from(action == 0 && count == 0);
            count += 1;
            count -= action;
        }

        Self::exp_mod(2, groups) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        Self::count_ways(ranges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
