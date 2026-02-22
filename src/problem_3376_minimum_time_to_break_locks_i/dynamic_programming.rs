pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn iter_bits(mut x: u8, mut f: impl FnMut(u8)) {
        while x != 0 {
            let bit = x & x.wrapping_neg();

            f(bit);

            x ^= bit;
        }
    }

    pub fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
        let strength = strength.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();
        let cache = &mut [0_u32; 256][..1 << strength.len()];

        for i in 1..cache.len() {
            let factor = 1 + k * (i.count_ones() - 1);
            let mut cost = u32::MAX;

            Self::iter_bits(i as _, |bit| {
                cost = cost
                    .min(cache[usize::from(i as u8 ^ bit)] + strength[bit.trailing_zeros() as usize].div_ceil(factor));
            });

            cache[i] = cost;
        }

        cache.last().unwrap().cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
        Self::find_minimum_time(strength, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
