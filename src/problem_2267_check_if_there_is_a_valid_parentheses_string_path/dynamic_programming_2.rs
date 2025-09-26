pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::{BitAnd, BitOrAssign, ShlAssign, ShrAssign};

impl Solution {
    fn helper<T, const N: usize>(zero: T, one: T, grid: Vec<Vec<char>>) -> bool
    where
        T: BitAnd<Output = T> + BitOrAssign + Copy + Eq + ShlAssign + ShrAssign,
    {
        let mut state = [zero; N];

        state[0] = one;

        let mut left = zero;

        for row in grid {
            left = zero;

            for (top, c) in state.iter_mut().zip(row) {
                *top |= left;

                if c == '(' {
                    *top <<= one;
                } else {
                    *top >>= one;
                }

                left = *top;
            }
        }

        left & one != zero
    }

    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        if (grid.len() + grid.first().map_or(0, Vec::len)).is_multiple_of(2) {
            false
        } else {
            match grid.first().map_or(0, Vec::len) {
                0..=8 => Self::helper::<u8, 8>(0, 1, grid),
                9..=16 => Self::helper::<u16, 16>(0, 1, grid),
                17..=32 => Self::helper::<u32, 32>(0, 1, grid),
                33..=64 => Self::helper::<u64, 64>(0, 1, grid),
                _ => Self::helper::<u128, 100>(0, 1, grid),
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        Self::has_valid_path(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
