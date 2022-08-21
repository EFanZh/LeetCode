pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(counts: &mut [u32; 26], result: &mut u32) {
        *result += 1;

        for i in 0..counts.len() {
            let count = &mut counts[i];

            if *count != 0 {
                *count -= 1;
                Self::helper(counts, result);
                counts[i] += 1;
            }
        }
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counts = [0_u32; 26];

        for c in tiles.into_bytes() {
            counts[usize::from(c) - usize::from(b'A')] += 1;
        }

        let mut result = 0;

        Self::helper(&mut counts, &mut result);

        (result - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_tile_possibilities(tiles: String) -> i32 {
        Self::num_tile_possibilities(tiles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
