pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn add(lhs: u32, rhs: u32) -> u32 {
        let result = lhs + rhs;

        result.checked_sub(Self::MODULUS).unwrap_or(result)
    }

    fn combinations(cache: &mut HashMap<(u16, u16), u32>, mut state: (u16, u16)) -> u32 {
        state.1 = state.1.min(state.0 - state.1);

        if let Some(&result) = cache.get(&state) {
            result
        } else {
            let result = if state.1 == 0 {
                1
            } else if state.1 == 1 {
                u32::from(state.0)
            } else {
                Self::add(
                    Self::combinations(cache, (state.0 - 1, state.1 - 1)),
                    Self::combinations(cache, (state.0 - 1, state.1)),
                )
            };

            cache.insert(state, result);

            result
        }
    }

    fn query(cache: &mut HashMap<(u16, u16), u32>, n: u16, mut k: u16) -> u32 {
        const PRIMES: [u16; 25] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        ];

        let n_minus_1 = n - 1;
        let mut result = 1;

        for prime in PRIMES {
            if prime > k {
                break;
            }

            let mut count = 0;

            while k % prime == 0 {
                k /= prime;
                count += 1;
            }

            result *= u64::from(Self::combinations(cache, (count + n_minus_1, n_minus_1)));
            result %= u64::from(Self::MODULUS);
        }

        if k != 1 {
            result *= u64::from(n);
            result %= u64::from(Self::MODULUS);
        }

        result as _
    }

    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cache = HashMap::default();

        queries
            .into_iter()
            .map(|query| {
                let [n, k]: [_; 2] = query.try_into().ok().unwrap();

                Self::query(&mut cache, n as _, k as _) as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::ways_to_fill_array(queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
