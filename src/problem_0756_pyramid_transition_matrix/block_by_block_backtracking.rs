pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

const MAX_BOTTOM_LENGTH: usize = 6;
const MAX_LETTERS: u8 = 6;

const MAX_BLOCKS: usize = (1 + MAX_BOTTOM_LENGTH) * MAX_BOTTOM_LENGTH / 2;
const SHIFT_BITS: u32 = 8 - MAX_LETTERS.leading_zeros();
const MAX_PAIRS: usize = (8 * (MAX_LETTERS - 1) + MAX_LETTERS) as _;
const LEFT_CHILD_INDICES: [usize; 15] = [1, 3, 4, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18, 19];

impl Solution {
    fn helper(blocks: &mut [u8; MAX_BLOCKS], index: usize, allowed: &[u8; MAX_PAIRS]) -> bool {
        if index == usize::MAX {
            true
        } else {
            let left_index = LEFT_CHILD_INDICES[index];
            let right_index = left_index + 1;
            let mut tops = allowed[usize::from((blocks[left_index] << SHIFT_BITS) | blocks[right_index])];

            while tops != 0 {
                blocks[index] = tops.trailing_zeros() as _;

                if Self::helper(blocks, index.wrapping_sub(1), allowed) {
                    return true;
                }

                tops &= tops - 1;
            }

            false
        }
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut allowed_map = [0; MAX_PAIRS];

        for triple in &allowed {
            let [left, right, top]: [u8; 3] = triple.as_bytes().try_into().unwrap();

            allowed_map[usize::from(((left - b'A') << SHIFT_BITS) | (right - b'A'))] |= 1 << (top - b'A');
        }

        let n = bottom.len();
        let mut blocks = [0; MAX_BLOCKS];
        let last_row_start = (n - 1) * n / 2;

        for (target, c) in blocks[last_row_start..].iter_mut().zip(bottom.bytes()) {
            *target = c - b'A';
        }

        Self::helper(&mut blocks, last_row_start - 1, &allowed_map)
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
