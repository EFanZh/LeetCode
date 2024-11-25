pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

#[expect(clippy::struct_field_names, reason = "by-design")]
struct Cache {
    seq_12: u32,
    seq_13: u32,
    seq_14: u32,
    seq_15: u32,
    seq_16: u32,
    seq_21: u32,
    seq_23: u32,
    seq_25: u32,
    seq_31: u32,
    seq_32: u32,
    seq_34: u32,
    seq_35: u32,
    seq_41: u32,
    seq_43: u32,
    seq_45: u32,
    seq_51: u32,
    seq_52: u32,
    seq_53: u32,
    seq_54: u32,
    seq_56: u32,
    seq_61: u32,
    seq_65: u32,
}

impl Cache {
    fn new(value: u32) -> Self {
        Self {
            seq_12: value,
            seq_13: value,
            seq_14: value,
            seq_15: value,
            seq_16: value,
            seq_21: value,
            seq_23: value,
            seq_25: value,
            seq_31: value,
            seq_32: value,
            seq_34: value,
            seq_35: value,
            seq_41: value,
            seq_43: value,
            seq_45: value,
            seq_51: value,
            seq_52: value,
            seq_53: value,
            seq_54: value,
            seq_56: value,
            seq_61: value,
            seq_65: value,
        }
    }
}

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn add_2(x: u32, y: u32) -> u32 {
        let candidate = x + y;

        candidate.checked_sub(Self::MODULUS).unwrap_or(candidate)
    }

    fn add_3(x: u32, y: u32, z: u32) -> u32 {
        Self::add_2(Self::add_2(x, y), z)
    }

    fn add_4(x: u32, y: u32, z: u32, w: u32) -> u32 {
        Self::add_2(Self::add_2(x, y), Self::add_2(z, w))
    }

    pub fn distinct_sequences(n: i32) -> i32 {
        // Valid sequences:
        //
        // 12, 13, 14, 15, 16, 21, 23, 25, 31, 32, 34, 35, 41, 43, 45, 51, 52, 53, 54, 56, 61, 65.

        let n = n as u32;

        if n < 3 {
            // Lagrange Interpolation.

            return (n * (11 * n).wrapping_sub(1) / 2 + 1) as _;
        }

        let mut cache = &mut Cache::new(1);
        let mut temp = &mut Cache::new(0);

        // It’s possible to use matrix multiplication to get a O(log(n)) time algorithm, but the constant factor is too
        // large for the testing range.
        for _ in 2..n {
            *temp = Cache {
                seq_12: Self::add_4(cache.seq_31, cache.seq_41, cache.seq_51, cache.seq_61),
                seq_13: Self::add_4(cache.seq_21, cache.seq_41, cache.seq_51, cache.seq_61),
                seq_14: Self::add_4(cache.seq_21, cache.seq_31, cache.seq_51, cache.seq_61),
                seq_15: Self::add_4(cache.seq_21, cache.seq_31, cache.seq_41, cache.seq_61),
                seq_16: Self::add_4(cache.seq_21, cache.seq_31, cache.seq_41, cache.seq_51),
                seq_21: Self::add_2(cache.seq_32, cache.seq_52),
                seq_23: Self::add_2(cache.seq_12, cache.seq_52),
                seq_25: Self::add_2(cache.seq_12, cache.seq_32),
                seq_31: Self::add_3(cache.seq_23, cache.seq_43, cache.seq_53),
                seq_32: Self::add_3(cache.seq_13, cache.seq_43, cache.seq_53),
                seq_34: Self::add_3(cache.seq_13, cache.seq_23, cache.seq_53),
                seq_35: Self::add_3(cache.seq_13, cache.seq_23, cache.seq_43),
                seq_41: Self::add_2(cache.seq_34, cache.seq_54),
                seq_43: Self::add_2(cache.seq_14, cache.seq_54),
                seq_45: Self::add_2(cache.seq_14, cache.seq_34),
                seq_51: Self::add_4(cache.seq_25, cache.seq_35, cache.seq_45, cache.seq_65),
                seq_52: Self::add_4(cache.seq_15, cache.seq_35, cache.seq_45, cache.seq_65),
                seq_53: Self::add_4(cache.seq_15, cache.seq_25, cache.seq_45, cache.seq_65),
                seq_54: Self::add_4(cache.seq_15, cache.seq_25, cache.seq_35, cache.seq_65),
                seq_56: Self::add_4(cache.seq_15, cache.seq_25, cache.seq_35, cache.seq_45),
                seq_61: cache.seq_56,
                seq_65: cache.seq_16,
            };

            mem::swap(&mut cache, &mut temp);
        }

        ((u64::from(cache.seq_12)
            + u64::from(cache.seq_13)
            + u64::from(cache.seq_14)
            + u64::from(cache.seq_15)
            + u64::from(cache.seq_16)
            + u64::from(cache.seq_21)
            + u64::from(cache.seq_23)
            + u64::from(cache.seq_25)
            + u64::from(cache.seq_31)
            + u64::from(cache.seq_32)
            + u64::from(cache.seq_34)
            + u64::from(cache.seq_35)
            + u64::from(cache.seq_41)
            + u64::from(cache.seq_43)
            + u64::from(cache.seq_45)
            + u64::from(cache.seq_51)
            + u64::from(cache.seq_52)
            + u64::from(cache.seq_53)
            + u64::from(cache.seq_54)
            + u64::from(cache.seq_56)
            + u64::from(cache.seq_61)
            + u64::from(cache.seq_65))
            % u64::from(Self::MODULUS)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distinct_sequences(n: i32) -> i32 {
        Self::distinct_sequences(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
