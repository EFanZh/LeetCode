pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn helper(text: &[u8], start: usize, diff: usize) -> u32 {
        let mut i = start;

        let iter_1 = iter::from_fn(move || {
            text.get(i).map(|&c| {
                i = i.wrapping_add(diff);

                usize::from(c) - usize::from(b'a')
            })
        });

        let iter_2 = iter_1.clone();
        let mut counts = [0_u32; 26];

        for c in iter_1 {
            counts[c] += 1;
        }

        let mut result = 0;
        let mut block_1 = (0, 0);
        let mut block_2 = (0, 0);
        let mut block_3 = (0, 0);

        for c in iter_2 {
            counts[c] -= 1;

            if c == block_3.0 {
                // Update blocks.

                block_3.1 += 1;
            } else {
                // Append to a block.

                result = result.max(block_3.1 + u32::from(counts[block_3.0] != 0));

                // Join two blocks.

                if block_1.0 == block_3.0 && block_2.1 == 1 {
                    result = result.max(block_1.1 + block_3.1 + u32::from(counts[block_1.0] != 0));
                }

                // Update blocks.

                block_1 = block_2;
                block_2 = block_3;
                block_3 = (c, 1);
            }
        }

        // Join two blocks.

        if block_1.0 == block_3.0 && block_2.1 == 1 {
            result = result.max(block_1.1 + block_3.1);
        }

        result.max(block_3.1)
    }

    pub fn max_rep_opt1(text: String) -> i32 {
        let text = text.as_bytes();
        let forward = Self::helper(text, 0, 1);
        let backward = Self::helper(text, text.len().wrapping_sub(1), usize::MAX);

        forward.max(backward) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_rep_opt1(text: String) -> i32 {
        Self::max_rep_opt1(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
