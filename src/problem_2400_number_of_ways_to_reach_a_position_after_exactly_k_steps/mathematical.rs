pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn exp_mod(mut base: u64, mut exponent: u32) -> u64 {
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

    fn mod_inverse(x: u64) -> u64 {
        Self::exp_mod(x, Self::MODULUS as u32 - 2)
    }

    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        let k = k as u32;
        let distance = start_pos.abs_diff(end_pos);

        match k.cmp(&distance) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => {
                let k_plus_distance = k + distance;

                if k_plus_distance % 2 == 0 {
                    let t = k_plus_distance / 2;
                    let mut factorial = 1;

                    let factorials = (1..=u64::from(k))
                        .map(|i| {
                            let result = factorial as u32;

                            factorial = (factorial * i) % Self::MODULUS;

                            result
                        })
                        .collect::<Box<_>>();

                    (factorial
                        * Self::mod_inverse(
                            u64::from(factorials[t as usize]) * u64::from(factorials[(k - t) as usize]) % Self::MODULUS,
                        )
                        % Self::MODULUS) as _
                } else {
                    0
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        Self::number_of_ways(start_pos, end_pos, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
