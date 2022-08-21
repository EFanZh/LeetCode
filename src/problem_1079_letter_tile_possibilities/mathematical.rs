pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn factorial(n: usize) -> u32 {
        const FACTORIALS: [u32; 7] = [1, 1, 2, 6, 24, 120, 720];

        FACTORIALS.get(n).copied().unwrap_or(5040)
    }

    fn helper(buffer: &mut [u32], i: usize, result: &mut u32) {
        if let Some(&count) = buffer.get(i) {
            for selected in 0..=count {
                buffer[i] = selected;
                Self::helper(buffer, i + 1, result);
            }

            buffer[i] = count;
        } else {
            let mut total_count = 0;
            let mut denominator = 1;

            for &mut count in buffer {
                total_count += count;
                denominator *= Self::factorial(count as _);
            }

            *result += Self::factorial(total_count as _) / denominator;
        }
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counts = [0_u32; 26];

        for c in tiles.into_bytes() {
            counts[usize::from(c) - usize::from(b'A')] += 1;
        }

        let mut i = 0;
        let mut length = 0;

        while let Some(&count) = counts.get(i) {
            if count != 0 {
                counts[length] = count;
                length += 1;
            }

            i += 1;
        }

        let mut result = 0;

        Self::helper(&mut counts[..length], 0, &mut result);

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
