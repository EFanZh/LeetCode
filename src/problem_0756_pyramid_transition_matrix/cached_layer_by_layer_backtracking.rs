pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::convert::TryInto;

const MAX_BOTTOM_LENGTH: usize = 6;
const MAX_LETTERS: u8 = 6;

const MAX_BLOCKS: usize = (MAX_BOTTOM_LENGTH - 1) * MAX_BOTTOM_LENGTH / 2;
const SHIFT_BITS: u32 = 8 - MAX_LETTERS.leading_zeros();
const MAX_PAIRS: usize = (8 * (MAX_LETTERS - 1) + MAX_LETTERS) as _;

impl Solution {
    fn helper_cell(
        bottom: &[u8],
        blocks: &mut [u8],
        index: usize,
        allowed: &[u8; MAX_PAIRS],
        f: &mut impl FnMut(&[u8], &mut [u8]) -> bool,
    ) -> bool {
        let row = bottom.len() - 1;

        if index == row {
            let (left, right) = blocks.split_at_mut(row);

            f(left, right)
        } else {
            let mut tops = allowed[usize::from((bottom[index] << SHIFT_BITS) | bottom[index + 1])];

            while tops != 0 {
                blocks[index] = tops.trailing_zeros() as _;

                if Self::helper_cell(bottom, blocks, index + 1, allowed, f) {
                    return true;
                }

                tops &= tops - 1;
            }

            false
        }
    }

    fn helper_layer(
        bottom: &[u8],
        blocks: &mut [u8],
        allowed: &[u8; MAX_PAIRS],
        cache: &mut HashMap<Box<[u8]>, bool>,
    ) -> bool {
        if bottom.len() == 2 {
            allowed[usize::from((bottom[0] << SHIFT_BITS) | bottom[1])] != 0
        } else if let Some(&result) = cache.get(bottom) {
            result
        } else {
            let result = Self::helper_cell(bottom, blocks, 0, allowed, &mut |next_bottom, next_blocks| {
                Self::helper_layer(next_bottom, next_blocks, allowed, cache)
            });

            cache.insert(bottom.into(), result);

            result
        }
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut allowed_map = [0; MAX_PAIRS];

        for triple in &allowed {
            let [left, right, top]: [u8; 3] = triple.as_bytes().try_into().unwrap();

            allowed_map[usize::from(((left - b'A') << SHIFT_BITS) | (right - b'A'))] |= 1 << (top - b'A');
        }

        let mut bottom = bottom.into_bytes();

        for c in &mut bottom {
            *c -= b'A';
        }

        let mut blocks = [0; MAX_BLOCKS - 1];

        Self::helper_layer(&bottom, &mut blocks, &allowed_map, &mut HashMap::default())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        Self::pyramid_transition(bottom, allowed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
