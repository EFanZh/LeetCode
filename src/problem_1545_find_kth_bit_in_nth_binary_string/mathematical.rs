pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let _ = n;
        let k = k.cast_unsigned();
        let last_bit = k.isolate_lowest_one();
        let last_chunk_of_bits = k & !(k + last_bit);

        if (last_bit == 1) == (last_bit == last_chunk_of_bits) {
            '0'
        } else {
            '1'
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_kth_bit(n: i32, k: i32) -> char {
        Self::find_kth_bit(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
