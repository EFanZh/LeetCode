pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    fn exp_mod(mut base: u64, mut exponent: u32, modulus: NonZero<u64>) -> u64 {
        let mut result = 1;

        if exponent != 0 {
            loop {
                if exponent & 1 != 0 {
                    result = (result * base) % modulus;

                    if exponent == 1 {
                        break;
                    }
                }

                exponent >>= 1;
                base = (base * base) % modulus;
            }
        }

        result
    }

    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        variables
            .into_iter()
            .enumerate()
            .filter_map(|(index, item)| {
                let [a, b, c, m] = item.try_into().ok().unwrap();

                let actual = Self::exp_mod(
                    Self::exp_mod(
                        u64::from(a.cast_unsigned()),
                        b.cast_unsigned(),
                        NonZero::new(10).unwrap(),
                    ),
                    c.cast_unsigned(),
                    NonZero::new(u64::from(m.cast_unsigned())).unwrap(),
                );

                (actual as i32 == target).then_some(index as _)
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        Self::get_good_indices(variables, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
