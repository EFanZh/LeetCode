pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let (left, right) = blocks.split_at(k as u32 as _);
        let mut white_count = left.iter().fold(0, |count, &c| count + u8::from(c == b'W'));
        let mut result = white_count;

        for (&old, &new) in blocks.iter().zip(right) {
            white_count -= u8::from(old == b'W');
            white_count += u8::from(new == b'W');
            result = result.min(white_count);
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_recolors(blocks: String, k: i32) -> i32 {
        Self::minimum_recolors(blocks, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
